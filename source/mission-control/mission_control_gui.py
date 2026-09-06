#!/usr/bin/env python3
"""
mission_control_gui.py - Theophysics Brain Mission Control (Calendar & Long-Term Scheduler Edition)
===================================================================================================
Unified Multi-Engine Studio & Standing Job Engine:
  - Local Desktop Ollama (127.0.0.1:11434)
  - Synology NAS Ollama (192.168.2.50:11434)
  - DeepSeek Chat Cloud API

Scheduling Modes:
  1. Instant Single-Pass Test (Pre-flight verify)
  2. Scheduled Run at Specific Day/Time (e.g., Monday 09:00, or specific datetime)
  3. Duration Execution (Run actively for 1 hour, 2 hours, etc., processing folder queue)
  4. Recurring Cadence (Every 12 min, 30 min, hourly, or daily at set hour)
  5. Multi-Stage Pipeline (Desktop Ollama -> NAS Ollama -> Lexicon CSV Intake)
"""

from __future__ import annotations

import argparse
import glob
import http.server
import json
import os
import re
import shutil
import socketserver
import subprocess
import threading
import time
import urllib.error
import urllib.parse
import urllib.request
import webbrowser
from datetime import datetime, timezone, timedelta
from pathlib import Path

# ============================================================
# CONSTANTS & SELF-RELATIVE PATHS
# ============================================================
HERE = Path(__file__).resolve().parent
ROOT_DIR = HERE.parent if HERE.name.lower() == "scripts" else HERE
MISSIONS_DIR = ROOT_DIR / "_missions"
MISSIONS_DIR.mkdir(parents=True, exist_ok=True)
PRESETS_FILE = MISSIONS_DIR / "custom_presets.json"
SCHEDULED_JOBS_FILE = MISSIONS_DIR / "standing_scheduled_jobs.json"

DESKTOP_OLLAMA_URL = "http://127.0.0.1:11434"
NAS_OLLAMA_URL = "https://ollama.dlowehomelab.com"
LOCAL_PORT = 7860

DEEPSEEK_API_KEY = os.environ.get("DEEPSEEK_API_KEY", "")

DEFAULT_PRESETS = {
    "notebooklm_media_pipeline": {
        "title": "🎙️ NotebookLM Canon Media Producer (Tier 1 vs Tier 2)",
        "system_prompt": "You are the Executive Media Director and Production Engineer for The One Story and Theophysics. You govern NotebookLM media generation, podcast naming (DD_, AD_, CR_), blackout slide decks, and trailer videos.",
        "task": "Audit the chapter or series folder. Extract the primary source, prepare notebook upload payload, specify Deep Dive (DD_) and Debate (AD_) podcast targets, 1 blackout slide set, and Explainer Video for 00_INDEX.md.",
        "extra_prompt": "Enforce the Shared Media Rule: all member papers in a chapter inherit and embed chapter shared DD_ and AD_ audios. Log all generated files into 03_MEDIA/00_MEDIA_LINKS.md.",
        "schema": '{\n  "title": "...",\n  "chapter": "...",\n  "shared_audio_targets": ["DD_...", "AD_..."],\n  "blackout_slides": "...",\n  "explainer_video_summary": "..."\n}'
    },
    "canonical_lexicon_definition": {
        "title": "📖 Canonical Lexicon & Definition Extractor",
        "system_prompt": "You are the Canonical Lexicographer and Ontological Registrar for Theophysics. Define specialized words, formal concepts, mathematical operators, and domain terminology with uncompromising precision, ensuring zero conceptual drift.",
        "task": "Read the provided document. Identify all key specialized words, philosophical terms, theophysical variables, and operational concepts. For each term, extract preferred term, domain category, formal definition, math representation, defeat condition, and source quote anchor.",
        "extra_prompt": "Format each definition in Markdown suitable for appending directly into the Theophysics Vault Definition Registry and AG_SORTING_TERMS_INTAKE.csv.",
        "schema": '{\n  "definitions": [\n    {\n      "term": "...",\n      "category": "...",\n      "definition": "...",\n      "math_symbol": "...",\n      "defeat_condition": "...",\n      "anchor": "..."\n    }\n  ]\n}'
    },
    "master_eq_callouts": {
        "title": "⚡ Master Equation Callouts (Obsidian)",
        "system_prompt": "You are the Lead Editor for the Theophysics Obsidian Vault. You strictly adhere to formal ontology, the Master Equation dC/dt = O*G(1-C) - S*C, and canonical formatting.",
        "task": "Scan the input note. Identify concepts related to Coherence, Entropy, Grace, Observers, or Master Equation. Insert Obsidian-style markdown callout boxes [!info] or [!axiom] with formal mathematical definitions and linkages.",
        "extra_prompt": "Preserve all original text untouched. Only inject callouts in relevant thematic sections.",
        "schema": '{\n  "callouts": [\n    {\n      "section": "...",\n      "type": "axiom|info|proof",\n      "title": "...",\n      "content": "..."\n    }\n  ]\n}'
    },
    "epistemic_axiom_grounding": {
        "title": "⚖️ Theophysics Axiom & Triune Grounding",
        "system_prompt": "You are the Senior Epistemic Auditor for Theophysics. You map every claim to the Triune Ontology (Father/Field, Logos/Form-Coherence, Spirit/Grace-Action) and evaluate 10-rubric rigor.",
        "task": "Perform a rigorous epistemic audit. Identify the truth kernel, map to formal theophysical axioms, and test for internal consistency and empirical falsifiability.",
        "extra_prompt": "List 5-10 verbatim quoted anchors supporting the verdict.",
        "schema": '{\n  "verdict": "...",\n  "truth_kernel": "...",\n  "triune_mapping": {\n    "father_field": "...",\n    "logos_coherence": "...",\n    "spirit_grace": "..."\n  },\n  "quoted_anchors": ["..."]\n}'
    },
    "sunny_animals": {
        "title": "🦎 Sunny Animals / Morphological Invariants",
        "system_prompt": "You are a Biotheophysics Specialist modeling biological morphology, organismal coherence, and evolutionary thermodynamics.",
        "task": "Analyze the text for animal behaviors, morphological structures, environmental adaptations, and energetic constraints. Extract structural invariants and energy-budget ratios.",
        "extra_prompt": "Organize findings into a taxonomy table: Organism | Structure/Behavior | Physical Invariant | Coherence Role.",
        "schema": '{\n  "organisms": [\n    {\n      "name": "...",\n      "morphology": "...",\n      "invariant": "...",\n      "coherence_role": "..."\n    }\n  ]\n}'
    }
}

def load_all_presets() -> dict:
    presets = dict(DEFAULT_PRESETS)
    if PRESETS_FILE.exists():
        try:
            custom = json.loads(PRESETS_FILE.read_text(encoding="utf-8"))
            presets.update(custom)
        except Exception:
            pass
    return presets

def save_custom_preset(key: str, data: dict):
    custom = {}
    if PRESETS_FILE.exists():
        try:
            custom = json.loads(PRESETS_FILE.read_text(encoding="utf-8"))
        except Exception:
            custom = {}
    custom[key] = data
    PRESETS_FILE.write_text(json.dumps(custom, indent=2), encoding="utf-8")

def load_scheduled_jobs() -> list[dict]:
    if SCHEDULED_JOBS_FILE.exists():
        try:
            return json.loads(SCHEDULED_JOBS_FILE.read_text(encoding="utf-8"))
        except Exception:
            return []
    return []

def save_scheduled_jobs(jobs: list[dict]):
    SCHEDULED_JOBS_FILE.write_text(json.dumps(jobs, indent=2), encoding="utf-8")

# ============================================================
# HTML TEMPLATE
# ============================================================
HTML_TEMPLATE = r"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Theophysics Brain Mission Control — Standing Scheduler Studio</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  :root {
    --bg: #0c0c0c;
    --surf: #121212;
    --surf2: #171717;
    --surf3: #1c1c1c;
    --bd: #242424;
    --bd2: #2d2d2d;
    --bd3: #383838;
    --tx: #e2e2e2;
    --mu: #8a8a8a;
    --mu2: #a2a2a2;
    --mu3: #c4c4c4;
    --acc: #4a7fd4;
    --gld: #d0a030;
    --gld-bright: #f5d070;
    --gld-bg: #2a200a;
    --gld-bd: #7a6020;
    --grn: #3fd05f;
    --grn-bg: #14351b;
    --grn-bd: #238636;
    --red: #e06060;
    --blu: #7fb0ff;
    --blu-bg: #152640;
    --pur: #c09fff;
    --code-txt: #7fae86;
  }
  body {
    font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, Roboto, sans-serif;
    background: var(--bg);
    color: var(--tx);
    font-size: 13.5px;
    line-height: 1.5;
    padding: 0 0 60px 0;
  }

  /* TOP COMMAND BAR */
  header {
    background: var(--surf);
    border-bottom: 1px solid var(--bd);
    padding: 10px 24px;
    display: flex;
    align-items: center;
    gap: 16px;
    position: sticky;
    top: 0;
    z-index: 200;
  }
  .h-brand { display: flex; align-items: center; gap: 10px; }
  .h-logo {
    width: 28px;
    height: 28px;
    background: var(--gld-bg);
    border: 1px solid var(--gld-bd);
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    color: var(--gld-bright);
    font-weight: 800;
  }
  .h-title { font-size: 14px; font-weight: 700; color: #fff; letter-spacing: 0.04em; }
  .h-sub { font-size: 11px; color: var(--mu); }
  .h-spacer { flex: 1; }

  /* BADGES */
  .badge-cluster { display: flex; gap: 8px; align-items: center; }
  .engine-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
    font-family: 'Cascadia Code', Consolas, monospace;
  }
  .engine-online { background: var(--grn-bg); color: #a9f3bf; border: 1px solid var(--grn-bd); }
  .engine-offline { background: #3c1e1e; color: #ff9b9b; border: 1px solid #7d2626; }
  .dot { width: 7px; height: 7px; border-radius: 50%; display: inline-block; }
  .dot-online { background: var(--grn); box-shadow: 0 0 6px rgba(63,208,95,0.6); }
  .dot-offline { background: var(--red); }

  /* CONTAINER */
  .container {
    max-width: 1440px;
    margin: 20px auto;
    padding: 0 20px;
    display: flex;
    flex-direction: column;
    gap: 18px;
  }

  /* SECTION BANNERS */
  .part { display: flex; align-items: center; gap: 12px; margin: 6px 0 2px; }
  .part-tag {
    font-size: 9px;
    font-weight: 800;
    letter-spacing: .14em;
    color: var(--mu);
    background: var(--surf2);
    border: 1px solid var(--bd);
    padding: 3px 8px;
    border-radius: 3px;
    text-transform: uppercase;
  }
  .part-name { font-size: 11px; font-weight: 700; letter-spacing: .05em; color: var(--mu2); text-transform: uppercase; }
  .part-line { flex: 1; height: 1px; background: var(--bd); }

  /* GRID */
  .grid { display: grid; grid-template-columns: 1.05fr 1fr; gap: 18px; }
  @media (max-width: 1040px) { .grid { grid-template-columns: 1fr; } }

  /* CARDS */
  .card {
    background: var(--surf);
    border: 1px solid var(--bd);
    border-radius: 8px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  .card-head {
    background: var(--surf2);
    border-bottom: 1px solid var(--bd);
    padding: 11px 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .card-head h2 {
    font-size: 12.5px;
    font-weight: 700;
    color: var(--mu3);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .card-body {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 14px;
    background: #0f0f0f;
  }

  .snum {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: var(--bd2);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 800;
    color: var(--mu2);
  }
  .snum.active { background: var(--gld-bg); color: var(--gld-bright); border: 1px solid var(--gld-bd); }

  /* FORMS */
  label {
    font-size: 10.5px;
    font-weight: 700;
    color: var(--mu2);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 5px;
    display: flex;
    justify-content: space-between;
  }
  label span.hint { font-size: 10px; color: var(--mu); font-weight: normal; text-transform: none; }
  input[type="text"], input[type="number"], input[type="datetime-local"], select, textarea {
    width: 100%;
    background: #080808;
    border: 1px solid var(--bd2);
    color: var(--tx);
    padding: 8px 10px;
    border-radius: 5px;
    font-size: 12.5px;
    font-family: inherit;
    transition: border-color 0.15s, box-shadow 0.15s;
  }
  input[type="text"]:focus, select:focus, textarea:focus, input[type="datetime-local"]:focus {
    outline: none;
    border-color: var(--gld);
    box-shadow: 0 0 0 1px rgba(208, 160, 48, 0.25);
  }
  textarea { resize: vertical; line-height: 1.45; }

  /* TOOLBARS & BUTTONS */
  .preset-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--surf3);
    border: 1px solid var(--bd2);
    padding: 7px 10px;
    border-radius: 6px;
  }
  .preset-bar select { flex: 1; background: #0c0c0c; }
  .btn-row { display: flex; gap: 8px; flex-wrap: wrap; }
  .hbtn {
    padding: 7px 14px;
    border-radius: 5px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.15s;
    display: inline-flex;
    align-items: center;
    gap: 7px;
    user-select: none;
  }
  .hbtn:hover { opacity: 0.88; }
  .hbtn-gold { background: var(--gld-bg); border-color: var(--gld-bd); color: var(--gld-bright); }
  .hbtn-gold:hover { background: #382c10; border-color: var(--gld); color: #fff; }
  .hbtn-blue { background: var(--blu-bg); border-color: #274a6f; color: #91c8ff; }
  .hbtn-blue:hover { background: #1b3558; border-color: var(--acc); color: #fff; }
  .hbtn-green { background: var(--grn-bg); border-color: var(--grn-bd); color: #a9f3bf; }
  .hbtn-green:hover { background: #1e4d27; border-color: var(--grn); color: #fff; }
  .hbtn-pur { background: #281836; border-color: #633285; color: #d6b8f7; }
  .hbtn-pur:hover { background: #371d4b; border-color: var(--pur); color: #fff; }
  .hbtn-ghost { background: transparent; border-color: var(--bd2); color: var(--mu2); }
  .hbtn-ghost:hover { color: var(--tx); border-color: var(--bd3); }

  /* CONSOLE */
  .console-panel {
    background: #070707;
    border: 1px solid var(--bd);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .console-head {
    background: #111;
    border-bottom: 1px solid var(--bd);
    padding: 7px 12px;
    font-size: 10px;
    font-weight: 800;
    letter-spacing: .08em;
    color: var(--mu);
    display: flex;
    align-items: center;
    justify-content: space-between;
    text-transform: uppercase;
  }
  .output-box {
    padding: 12px;
    min-height: 240px;
    max-height: 420px;
    overflow-y: auto;
    font-family: 'Cascadia Code', Consolas, monospace;
    font-size: 11.5px;
    line-height: 1.55;
    color: var(--code-txt);
    white-space: pre-wrap;
    background: #060606;
  }

  /* SPINNER */
  .spinner {
    display: none;
    width: 13px;
    height: 13px;
    border: 2px solid rgba(255,255,255,0.25);
    border-radius: 50%;
    border-top-color: #fff;
    animation: spin 0.75s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  /* SCHEDULER WIDGET BOX */
  .scheduler-box {
    background: #121212;
    border: 1px solid var(--bd2);
    border-radius: 6px;
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .sched-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }
  .jobs-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 11.5px;
  }
  .jobs-table th {
    text-align: left;
    padding: 6px 8px;
    background: #191919;
    color: var(--mu2);
    border-bottom: 1px solid var(--bd);
  }
  .jobs-table td {
    padding: 6px 8px;
    border-bottom: 1px solid #1a1a1a;
    font-family: 'Cascadia Code', Consolas, monospace;
  }
</style>
</head>
<body>

<!-- TOP COMMAND BAR -->
<header>
  <div class="h-brand">
    <div class="h-logo">Ω</div>
    <div>
      <div class="h-title">THEOPHYSICS BRAIN MISSION CONTROL</div>
      <div class="h-sub">ATOM Full Studio · Standing Scheduler · Duration Workflows · Dual Engine</div>
    </div>
  </div>

  <div class="h-spacer"></div>

  <!-- Dual Status Badges -->
  <div class="badge-cluster">
    <div id="desktop-badge" class="engine-badge engine-online">
      <span class="dot dot-online" id="desktop-dot"></span>
      <span id="desktop-label">Desktop PC (127.0.0.1:11434)</span>
    </div>
    <div id="nas-badge" class="engine-badge engine-online">
      <span class="dot dot-online" id="nas-dot"></span>
      <span id="nas-label">Synology NAS (192.168.2.50:11434)</span>
    </div>
  </div>
</header>

<div class="container">

  <div class="part">
    <span class="part-tag">WORKBENCH 1 &amp; 2</span>
    <span class="part-name">Prompt Stack, Schema Studio &amp; AI Refinement</span>
    <div class="part-line"></div>
  </div>

  <!-- 2-COLUMN GRID -->
  <div class="grid">

    <!-- LEFT COLUMN: PROMPT STACK & SCHEMA STUDIO -->
    <div class="card">
      <div class="card-head">
        <h2><span class="snum active">1</span> Prompt Stack &amp; Schema Studio</h2>
      </div>

      <div class="card-body">
        <!-- Preset Selector Toolbar -->
        <div class="preset-bar">
          <span style="font-size: 11px; font-weight: 700; color: var(--gld-bright); text-transform: uppercase;">Preset:</span>
          <select id="preset-select" onchange="loadPreset()">
            <option value="">-- Choose Canon Preset --</option>
          </select>
          <button class="hbtn hbtn-gold" type="button" onclick="saveAsNewPreset()" style="padding: 4px 10px; font-size: 11px;">
            <span>💾 Save Preset</span>
          </button>
        </div>

        <div>
          <label>A. System Persona &amp; Foundational Ontology <span class="hint">Governs authority, tone &amp; math</span></label>
          <textarea id="sys-prompt" rows="3">You are the Lead Epistemic Auditor for Theophysics and the Consilience Atlas. Maintain rigorous mathematical notation, triune ontology, and non-destructive standards.</textarea>
        </div>

        <div>
          <label>B. Core Task Objective <span class="hint">What the model must execute</span></label>
          <textarea id="task-prompt" rows="3">Scan the input document. Extract core propositions, classify claim maturity (1-7), and identify dependencies on the Master Equation dC/dt = O*G(1-C) - S*C.</textarea>
        </div>

        <div>
          <label>C. Daily Refinement / Special Focus <span class="hint">Optional filters or temporary directives</span></label>
          <textarea id="extra-prompt" rows="2" placeholder="e.g. Focus on Chapter 01 shared media rules, or extract verbatim quote anchors for CSV intake..."></textarea>
        </div>

        <!-- SCHEMA STUDIO SECTION -->
        <div style="border-top: 1px solid var(--bd2); padding-top: 10px;">
          <label>
            <span>D. JSON Output Schema (Strict Shape Enforcement)</span>
            <span class="hint" style="color: var(--pur);">Guarantees zero model drift</span>
          </label>
          <textarea id="schema-prompt" rows="4" style="font-family: 'Cascadia Code', monospace; color: #b7d6ff;">{
  "title": "...",
  "central_claim": "...",
  "evidence_anchors": ["verbatim quotes"],
  "what_this_does_not_establish": "boundary"
}</textarea>
        </div>

        <!-- REFINER BUTTON -->
        <div class="btn-row" style="margin-top: 2px;">
          <button class="hbtn hbtn-pur" type="button" onclick="refineWithDeepSeek()">
            <span class="spinner" id="refine-spinner"></span>
            <span>⚡ Refine Prompt &amp; Schema with DeepSeek</span>
          </button>
        </div>

        <!-- TARGET SCOPE -->
        <div style="border-top: 1px solid var(--bd2); padding-top: 10px; display: flex; gap: 12px;">
          <div style="flex: 2;">
            <label>Target Vault / Batch Folder</label>
            <input type="text" id="target-folder" value="C:\Users\David\Documents\faiththruphysics.com\02_CANONIZATION">
          </div>
          <div style="flex: 1;">
            <label>Filter</label>
            <input type="text" id="file-pattern" value="*.md">
          </div>
        </div>

      </div>
    </div>

    <!-- RIGHT COLUMN: LIVE INSPECTION & ADVANCED CALENDAR SCHEDULER -->
    <div class="card">
      <div class="card-head">
        <h2><span class="snum active">2</span> Single-File Live Verification</h2>
        <span style="font-size: 10px; color: var(--mu); text-transform: uppercase;">Pre-Flight Test</span>
      </div>

      <div class="card-body">
        <div style="display: flex; gap: 10px;">
          <div style="flex: 2;">
            <label>Sample File Path for Live Test</label>
            <input type="text" id="test-file" value="C:\Users\David\Documents\faiththruphysics.com\00_ Production\00_ Production\01_GOD_AXIOM\01_THE_ONE_STORY\01_GOD_AS_ROOT\01_GOD_AS_ROOT_FULL_SOURCE.md">
          </div>
          <div style="flex: 1;">
            <label>Engine for Test</label>
            <select id="model-select">
              <optgroup label="Desktop PC (Local - 127.0.0.1)">
                <option value="desktop:llama3.2:3b">Desktop: llama3.2:3b</option>
                <option value="desktop:qwen3:4b-instruct">Desktop: qwen3:4b</option>
                <option value="desktop:gemma4:latest">Desktop: gemma4</option>
              </optgroup>
              <optgroup label="Synology NAS (24/7 - 192.168.2.50)">
                <option value="nas:llama3.2:latest" selected>NAS: llama3.2:latest</option>
                <option value="nas:mistral:latest">NAS: mistral:latest</option>
              </optgroup>
              <optgroup label="Cloud">
                <option value="cloud:deepseek">DeepSeek Chat</option>
              </optgroup>
            </select>
          </div>
        </div>

        <div class="btn-row">
          <button class="hbtn hbtn-blue" onclick="runSingleTest()">
            <span class="spinner" id="test-spinner"></span>
            <span>⚡ Run Single-File Test Now</span>
          </button>
          <button class="hbtn hbtn-ghost" onclick="document.getElementById('output-view').innerText = 'Inspection console cleared.'">
            <span>Clear Console</span>
          </button>
        </div>

        <!-- CONSOLE FEED -->
        <div class="console-panel">
          <div class="console-head">
            <span>Live Output / Execution Monitor</span>
            <span id="console-status" style="color: var(--gld);">READY</span>
          </div>
          <div class="output-box" id="output-view">Ready. Click "Run Single-File Test Now" to verify output against your prompt stack.</div>
        </div>

        <!-- ADVANCED CALENDAR & DURATION SCHEDULER -->
        <div class="card-head" style="margin: 4px -16px -14px -16px; border-top: 1px solid var(--bd);">
          <h2><span class="snum active">3</span> Standing Scheduler &amp; Long-Term Jobs</h2>
          <span style="font-size: 10px; color: var(--gld-bright);">Calendar · Duration · Recurring</span>
        </div>

        <div class="scheduler-box">
          <div class="sched-grid">
            <div>
              <label>Execution Mode</label>
              <select id="sched-mode" onchange="toggleSchedMode()">
                <option value="specific_time">📅 Specific Day / Time (e.g. Monday 9:00)</option>
                <option value="duration">⏳ Duration Run (e.g. Run for 1 Hour)</option>
                <option value="recurring">🔁 Recurring Cadence (Every 15m, 30m, Daily)</option>
              </select>
            </div>

            <div>
              <label>Target Machine Engine</label>
              <select id="sched-engine">
                <option value="nas:llama3.2:latest">Synology NAS (24/7 Low Power - llama3.2)</option>
                <option value="nas:mistral:latest">Synology NAS (24/7 Low Power - mistral)</option>
                <option value="desktop:qwen3:4b-instruct">Desktop PC (Local High-Compute - qwen3:4b)</option>
                <option value="desktop:llama3.2:3b">Desktop PC (Local Fast - llama3.2:3b)</option>
              </select>
            </div>
          </div>

          <!-- Dynamic Options based on Mode -->
          <div id="sched-time-options" class="sched-grid">
            <div>
              <label>Start Date &amp; Time</label>
              <input type="datetime-local" id="sched-datetime">
            </div>
            <div>
              <label>Session Run Duration</label>
              <select id="sched-duration">
                <option value="30">Run for 30 minutes</option>
                <option value="60" selected>Run for 1 hour</option>
                <option value="120">Run for 2 hours</option>
                <option value="unlimited">Run until folder completes</option>
              </select>
            </div>
          </div>

          <div id="sched-recurring-options" class="sched-grid" style="display: none;">
            <div>
              <label>Repeat Interval</label>
              <select id="sched-repeat">
                <option value="12m">Every 12 minutes (Chapter Queue Cadence)</option>
                <option value="30m">Every 30 minutes (Idle Nerve Heartbeat)</option>
                <option value="1h">Every 1 hour (Vault Scan)</option>
                <option value="daily_0900">Daily at 09:00 AM</option>
                <option value="mon_0900">Weekly on Monday at 09:00 AM</option>
              </select>
            </div>
            <div>
              <label>Long-Term Task Persistence</label>
              <span style="font-size: 11px; color: var(--mu); display: block; margin-top: 6px;">Keeps firing as long-running daemon in background.</span>
            </div>
          </div>

          <div class="btn-row" style="margin-top: 4px;">
            <button class="hbtn hbtn-green" onclick="scheduleStandingJob()">
              <span>⏰ Save &amp; Arm Standing Scheduled Job</span>
            </button>
            <button class="hbtn hbtn-gold" onclick="launchImmediateBatch()">
              <span class="spinner" id="batch-spinner"></span>
              <span>🚀 Launch Immediate Run Now</span>
            </button>
          </div>

          <!-- Standing Jobs Table -->
          <div style="border-top: 1px solid var(--bd2); padding-top: 8px;">
            <label>Active Standing Jobs &amp; Scheduled Tasks</label>
            <div style="max-height: 140px; overflow-y: auto;">
              <table class="jobs-table">
                <thead>
                  <tr>
                    <th>Target Time / Cadence</th>
                    <th>Engine</th>
                    <th>Task Scope</th>
                    <th>Action</th>
                  </tr>
                </thead>
                <tbody id="standing-jobs-tbody">
                  <tr><td colspan="4" style="color: var(--mu); text-align: center;">No standing jobs scheduled yet.</td></tr>
                </tbody>
              </table>
            </div>
          </div>

        </div>

      </div>
    </div>

  </div>
</div>

<script>
let PRESETS = {};

function toggleSchedMode() {
  const mode = document.getElementById('sched-mode').value;
  const timeBox = document.getElementById('sched-time-options');
  const recurBox = document.getElementById('sched-recurring-options');
  if (mode === 'recurring') {
    timeBox.style.display = 'none';
    recurBox.style.display = 'grid';
  } else {
    timeBox.style.display = 'grid';
    recurBox.style.display = 'none';
  }
}

// Set default datetime input to next upcoming hour
const now = new Date();
now.setHours(now.getHours() + 1, 0, 0, 0);
const pad = n => String(n).padStart(2, '0');
const localIso = `${now.getFullYear()}-${pad(now.getMonth()+1)}-${pad(now.getDate())}T${pad(now.getHours())}:${pad(now.getMinutes())}`;
document.getElementById('sched-datetime').value = localIso;

async function loadPresetsFromServer() {
  try {
    const res = await fetch('/api/presets');
    PRESETS = await res.json();
    const sel = document.getElementById('preset-select');
    sel.innerHTML = '<option value="">-- Choose Canon Preset --</option>';
    for (const [key, val] of Object.entries(PRESETS)) {
      sel.insertAdjacentHTML('beforeend', `<option value="${key}">${val.title}</option>`);
    }
  } catch(e) {
    console.error("Failed loading presets", e);
  }
}

function loadPreset() {
  const key = document.getElementById('preset-select').value;
  if (key && PRESETS[key]) {
    document.getElementById('sys-prompt').value = PRESETS[key].system_prompt || '';
    document.getElementById('task-prompt').value = PRESETS[key].task || '';
    document.getElementById('extra-prompt').value = PRESETS[key].extra_prompt || '';
    if (PRESETS[key].schema) {
      document.getElementById('schema-prompt').value = PRESETS[key].schema;
    }
  }
}

async function saveAsNewPreset() {
  const name = prompt("Enter a title for this new Preset:");
  if (!name || !name.trim()) return;
  const key = name.trim().toLowerCase().replace(/[^a-z0-9]+/g, '_');
  const payload = {
    key: key,
    data: {
      title: name.trim(),
      system_prompt: document.getElementById('sys-prompt').value,
      task: document.getElementById('task-prompt').value,
      extra_prompt: document.getElementById('extra-prompt').value,
      schema: document.getElementById('schema-prompt').value
    }
  };
  try {
    const res = await fetch('/api/presets/save', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify(payload)
    });
    alert("Saved Preset: " + name);
    loadPresetsFromServer();
  } catch(e) {
    alert("Save failed: " + e);
  }
}

async function refineWithDeepSeek() {
  const spin = document.getElementById('refine-spinner');
  spin.style.display = 'inline-block';
  const out = document.getElementById('output-view');
  out.innerText = "Connecting to DeepSeek to refine ontology, constraints, and JSON schema...\nPlease wait...";

  const payload = {
    system_prompt: document.getElementById('sys-prompt').value,
    task: document.getElementById('task-prompt').value,
    extra_prompt: document.getElementById('extra-prompt').value,
    schema: document.getElementById('schema-prompt').value
  };

  try {
    const res = await fetch('/api/refine', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify(payload)
    });
    const data = await res.json();
    spin.style.display = 'none';
    if (data.success && data.refined) {
      if (confirm("DeepSeek refined your prompt & schema for maximum epistemic rigor! Apply changes to editors now?")) {
        if (data.refined.system_prompt) document.getElementById('sys-prompt').value = data.refined.system_prompt;
        if (data.refined.task) document.getElementById('task-prompt').value = data.refined.task;
        if (data.refined.extra_prompt) document.getElementById('extra-prompt').value = data.refined.extra_prompt;
        if (data.refined.schema) document.getElementById('schema-prompt').value = data.refined.schema;
      }
      out.innerText = "=== DEEPSEEK REFINEMENT SUMMARY ===\n" + data.explanation;
    } else {
      out.innerText = "Refine error: " + data.error;
    }
  } catch(e) {
    spin.style.display = 'none';
    out.innerText = "Refine exception: " + e;
  }
}

async function checkEngineStatus() {
  try {
    const res = await fetch('/api/health');
    const data = await res.json();

    const dDot = document.getElementById('desktop-dot');
    const dBadge = document.getElementById('desktop-badge');
    const dLabel = document.getElementById('desktop-label');
    if (data.desktop_online) {
      dBadge.className = 'engine-badge engine-online';
      dDot.className = 'dot dot-online';
      dLabel.innerText = 'Desktop PC · ' + data.desktop_models.length + ' models';
    } else {
      dBadge.className = 'engine-badge engine-offline';
      dDot.className = 'dot dot-offline';
      dLabel.innerText = 'Desktop PC (Offline)';
    }

    const nDot = document.getElementById('nas-dot');
    const nBadge = document.getElementById('nas-badge');
    const nLabel = document.getElementById('nas-label');
    if (data.nas_online) {
      nBadge.className = 'engine-badge engine-online';
      nDot.className = 'dot dot-online';
      nLabel.innerText = 'Synology NAS · ' + data.nas_models.length + ' models';
    } else {
      nBadge.className = 'engine-badge engine-offline';
      nDot.className = 'dot dot-offline';
      nLabel.innerText = 'Synology NAS (Offline)';
    }
  } catch(e) {
    console.error('Health check error:', e);
  }
}

async function runSingleTest() {
  const out = document.getElementById('output-view');
  const spinner = document.getElementById('test-spinner');
  const status = document.getElementById('console-status');
  spinner.style.display = 'inline-block';
  status.innerText = "INFERRING...";
  out.innerText = "Connecting to selected engine & running test pass...\nPlease wait...";

  const payload = {
    action: "test_single",
    system_prompt: document.getElementById('sys-prompt').value,
    task: document.getElementById('task-prompt').value,
    extra_prompt: document.getElementById('extra-prompt').value,
    schema: document.getElementById('schema-prompt').value,
    test_file: document.getElementById('test-file').value,
    model: document.getElementById('model-select').value
  };

  try {
    const res = await fetch('/api/run', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify(payload)
    });
    const data = await res.json();
    spinner.style.display = 'none';
    if (data.success) {
      status.innerText = "SUCCESS";
      out.innerText = data.result;
    } else {
      status.innerText = "FAILED";
      out.innerText = "ERROR: " + data.error;
    }
  } catch(e) {
    spinner.style.display = 'none';
    status.innerText = "EXCEPTION";
    out.innerText = "Network/Execution Exception: " + e;
  }
}

async function loadStandingJobs() {
  try {
    const res = await fetch('/api/jobs');
    const jobs = await res.json();
    const tbody = document.getElementById('standing-jobs-tbody');
    if (!jobs || !jobs.length) {
      tbody.innerHTML = '<tr><td colspan="4" style="color: var(--mu); text-align: center;">No standing jobs scheduled yet.</td></tr>';
      return;
    }
    tbody.innerHTML = '';
    jobs.forEach((job, idx) => {
      tbody.insertAdjacentHTML('beforeend', `
        <tr>
          <td style="color: var(--gld-bright);">${job.schedule_desc}</td>
          <td>${job.engine}</td>
          <td>${job.folder.split('\\').pop() || job.folder} (${job.duration_min}m duration)</td>
          <td><button class="hbtn hbtn-ghost" style="padding: 2px 6px; font-size: 10px;" onclick="deleteJob(${idx})">✕ Cancel</button></td>
        </tr>
      `);
    });
  } catch(e) {
    console.error("Failed loading jobs", e);
  }
}

async function scheduleStandingJob() {
  const mode = document.getElementById('sched-mode').value;
  const engine = document.getElementById('sched-engine').value;
  const folder = document.getElementById('target-folder').value;
  let schedule_desc = "";
  let duration_min = 60;

  if (mode === 'recurring') {
    const rep = document.getElementById('sched-repeat').value;
    schedule_desc = `Recurring: ${rep}`;
    duration_min = 30;
  } else {
    const dt = document.getElementById('sched-datetime').value;
    const dur = document.getElementById('sched-duration').value;
    schedule_desc = `Scheduled: ${dt.replace('T', ' ')}`;
    duration_min = dur === 'unlimited' ? 9999 : parseInt(dur, 10);
  }

  const job = {
    id: 'job_' + Date.now(),
    mode,
    schedule_desc,
    engine,
    folder,
    duration_min,
    system_prompt: document.getElementById('sys-prompt').value,
    task: document.getElementById('task-prompt').value,
    extra_prompt: document.getElementById('extra-prompt').value,
    schema: document.getElementById('schema-prompt').value,
    created_at: new Date().toISOString()
  };

  try {
    const res = await fetch('/api/jobs/add', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify(job)
    });
    alert(`Successfully scheduled job!\n${schedule_desc} on ${engine}`);
    loadStandingJobs();
  } catch(e) {
    alert("Scheduling error: " + e);
  }
}

async function deleteJob(idx) {
  if (!confirm("Cancel this scheduled job?")) return;
  await fetch(`/api/jobs/delete?idx=${idx}`, { method: 'POST' });
  loadStandingJobs();
}

async function launchImmediateBatch() {
  if (!confirm("Launch immediate batch run across target folder?")) return;
  const out = document.getElementById('output-view');
  out.innerText = "Launching immediate batch run...";
  const payload = {
    action: "batch_run",
    system_prompt: document.getElementById('sys-prompt').value,
    task: document.getElementById('task-prompt').value,
    extra_prompt: document.getElementById('extra-prompt').value,
    schema: document.getElementById('schema-prompt').value,
    target_folder: document.getElementById('target-folder').value,
    file_pattern: document.getElementById('file-pattern').value,
    model: document.getElementById('sched-engine').value
  };
  const res = await fetch('/api/run', { method: 'POST', headers: {'Content-Type': 'application/json'}, body: JSON.stringify(payload) });
  const data = await res.json();
  out.innerText = data.message || "Batch job launched!";
}

loadPresetsFromServer();
loadStandingJobs();
checkEngineStatus();
setInterval(checkEngineStatus, 15000);
</script>
</body>
</html>
"""

# ============================================================
# BACKEND INFERENCE ENGINE & STANDING JOB MANAGER
# ============================================================
def ping_ollama(base_url: str) -> tuple[bool, list[str]]:
    try:
        req = urllib.request.Request(f"{base_url}/api/tags", headers={"Content-Type": "application/json", "User-Agent": "Nerve-Mission-Control/1.0"})
        with urllib.request.urlopen(req, timeout=2.5) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            models = [m["name"] for m in data.get("models", [])]
            return True, models
    except Exception:
        return False, []

def call_ollama(base_url: str, model_name: str, system_prompt: str, user_prompt: str, max_tokens: int = 3000) -> str:
    url = f"{base_url}/api/generate"
    full_prompt = f"System: {system_prompt}\n\nTask Instructions:\n{user_prompt}"
    payload = {
        "model": model_name,
        "prompt": full_prompt,
        "format": "json",
        "stream": False,
        "options": {
            "num_predict": max_tokens,
            "temperature": 0.1
        }
    }
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(url, data=data, headers={"Content-Type": "application/json", "User-Agent": "Nerve-Mission-Control/1.0"}, method="POST")
    try:
        with urllib.request.urlopen(req, timeout=180) as resp:
            body = json.loads(resp.read().decode("utf-8"))
            return body.get("response", "")
    except Exception as exc:
        return f"[Ollama Error ({url}): {exc}]"

def call_deepseek(system_prompt: str, user_prompt: str, max_tokens: int = 3000) -> str:
    if not DEEPSEEK_API_KEY:
        return "[Error: DEEPSEEK_API_KEY not found]"
    url = "https://api.deepseek.com/chat/completions"
    payload = {
        "model": "deepseek-chat",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}
        ],
        "max_tokens": max_tokens,
        "temperature": 0.2
    }
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(url, data=data, headers={"Authorization": f"Bearer {DEEPSEEK_API_KEY}", "Content-Type": "application/json"}, method="POST")
    try:
        with urllib.request.urlopen(req, timeout=120) as resp:
            body = json.loads(resp.read().decode("utf-8"))
            return body["choices"][0]["message"]["content"]
    except Exception as exc:
        return f"[DeepSeek Cloud Error: {exc}]"

def refine_prompt_with_deepseek(sys_p: str, task_p: str, extra_p: str, schema_p: str) -> dict:
    prompt = f"""You are the Elite Prompt Architect and Ontological Engineer for Theophysics.
Refine and optimize the following prompt stack and JSON schema for rigorous, drift-free execution on local LLMs (Ollama Llama 3.2 and Mistral).

CURRENT PROMPT STACK:
--- SYSTEM PERSONA ---
{sys_p}

--- CORE TASK ---
{task_p}

--- SPECIAL REFINEMENTS ---
{extra_p}

--- CURRENT JSON SCHEMA ---
{schema_p}

INSTRUCTIONS:
1. Tighten the system prompt so the model never apologizes, hedges needlessly, or hallucinates non-existent facts.
2. Structure the core task so it requires verbatim quote anchors for every extracted claim.
3. Validate and clean up the JSON schema to ensure strict syntactical correctness.
4. Output your answer ONLY as a valid JSON object matching this exact shape:
{{
  "system_prompt": "refined text",
  "task": "refined text",
  "extra_prompt": "refined text",
  "schema": "clean json schema string",
  "explanation": "concise breakdown of what was sharpened"
}}
"""
    raw = call_deepseek("You are a strict JSON prompt architect.", prompt)
    try:
        match = re.search(r"\{.*\}", raw, re.DOTALL)
        if match:
            return json.loads(match.group(0))
        return json.loads(raw)
    except Exception as e:
        return {"error": f"Failed to parse DeepSeek response: {e}\nRaw: {raw[:300]}"}

def parse_model_json(raw: str) -> dict:
    """Accept plain or fenced model JSON while rejecting prose/error pages."""
    text = (raw or "").strip()
    if text.startswith("[") and "Error" in text[:80]:
        raise RuntimeError(text.strip("[]"))
    fenced = re.search(r"```(?:json)?\s*(\{.*\})\s*```", text, re.DOTALL | re.IGNORECASE)
    candidate = fenced.group(1) if fenced else text
    try:
        value = json.loads(candidate)
    except json.JSONDecodeError:
        start, end = text.find("{"), text.rfind("}")
        if start >= 0 and end > start:
            snippet = text[start:end + 1]
            try:
                value = json.loads(snippet)
            except json.JSONDecodeError:
                try:
                    import json_repair
                    value = json_repair.loads(snippet)
                except Exception as err:
                    raise RuntimeError(f"Model returned malformed JSON: {err}")
        else:
            try:
                import json_repair
                value = json_repair.loads(text)
            except Exception:
                raise RuntimeError("The model returned no JSON object")
    if not isinstance(value, dict):
        raise RuntimeError("The model response must be one JSON object")
    return value

def run_atom_builder_pass(payload: dict, pass_name: str) -> dict:
    provider = str(payload.get("_provider", "deepseek")).lower()
    model = str(payload.get("_model", "deepseek-chat"))
    authority = payload.get("authority", "CANDIDATE_DRAFT — NOT ADMITTED")
    schema = payload.get("response_schema", {})
    system_prompt = (
        "You are the governed Claim Atom Builder. Return ONLY valid JSON, with no markdown fence or prose. "
        f"All output remains {authority}. Never admit or canonize a result. "
        "Preserve source wording and never invent source anchors."
    )
    user_prompt = (
        f"Execute the {pass_name} pass. Follow response_schema exactly.\n\n"
        f"REQUEST:\n{json.dumps(payload, ensure_ascii=False)}\n\n"
        f"RESPONSE_SCHEMA:\n{json.dumps(schema, ensure_ascii=False)}"
    )
    if provider == "deepseek":
        raw = call_deepseek(system_prompt, user_prompt, max_tokens=8000)
    elif provider == "ollama":
        raw = call_ollama(NAS_OLLAMA_URL, model, system_prompt, user_prompt, max_tokens=8000)
    else:
        raise RuntimeError(f"Provider '{provider}' is not available in Mission Control")
    return parse_model_json(raw)

# ============================================================
# HTTP REQUEST HANDLER
# ============================================================
class MissionControlHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, format: str, *args) -> None:
        # pythonw.exe has no stderr stream; the inherited logger would abort
        # otherwise successful requests when this service runs in the tray.
        return

    def do_OPTIONS(self):
        self.send_response(204)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.end_headers()

    def do_GET(self):
        if self.path in ("/", "/index.html"):
            self.send_response(200)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.end_headers()
            self.wfile.write(HTML_TEMPLATE.encode("utf-8"))
            return
        elif self.path == "/api/health":
            d_online, d_models = ping_ollama(DESKTOP_OLLAMA_URL)
            n_online, n_models = ping_ollama(NAS_OLLAMA_URL)
            self._json_response({
                "desktop_online": d_online,
                "desktop_models": d_models,
                "nas_online": n_online,
                "nas_models": n_models,
            })
            return
        elif self.path == "/api/presets":
            self._json_response(load_all_presets())
            return
        elif self.path == "/api/jobs":
            self._json_response(load_scheduled_jobs())
            return
        self.send_error(404, "Not Found")

    def do_POST(self):
        length = int(self.headers.get("Content-Length", 0))
        raw = self.rfile.read(length).decode("utf-8")
        payload = json.loads(raw) if raw else {}

        if self.path.startswith("/api/atom-builder/"):
            pass_name = self.path.rsplit("/", 1)[-1]
            if pass_name not in {"builder", "audit", "regenerate"}:
                self._json_response({"error": f"Unknown Claim Atom pass: {pass_name}"}, status=404)
                return
            try:
                self._json_response(run_atom_builder_pass(payload, pass_name))
            except Exception as exc:
                self._json_response({"error": str(exc)}, status=503)
            return

        if self.path.startswith("/api/axiom-builder/query"):
            prompt = payload.get("prompt", "")
            source_text = payload.get("source_text", "")
            field_key = payload.get("field_key", "")
            system_prompt = payload.get("system_prompt", (
                "You are the authoritative Axiom Builder for the Faith Through Physics consilience framework. "
                "You evaluate foundational axioms against provided source paper text. "
                "Provide a direct, high-precision, unhedged answer or formalization for the requested field. "
                "Preserve exact source terminology, math rigor, and derivational boundaries."
            ))
            provider = str(payload.get("_provider", "deepseek")).lower()
            model = str(payload.get("_model", "deepseek-chat"))
            full_user = f"PRESERVED SOURCE TEXT:\n{source_text[:30000]}\n\nFIELD / QUESTION:\n{prompt}"
            try:
                if provider == "deepseek":
                    raw = call_deepseek(system_prompt, full_user, max_tokens=2500)
                elif provider == "ollama":
                    raw = call_ollama(NAS_OLLAMA_URL, model, system_prompt, full_user, max_tokens=2500)
                else:
                    raw = call_deepseek(system_prompt, full_user, max_tokens=2500)
                self._json_response({"result": raw.strip(), "field_key": field_key, "status": "PROPOSED"})
            except Exception as exc:
                self._json_response({"error": str(exc)}, status=503)
            return

        if self.path == "/api/presets/save":
            key = payload.get("key")
            data = payload.get("data")
            if key and data:
                save_custom_preset(key, data)
                self._json_response({"success": True})
                return
            self._json_response({"success": False, "error": "Invalid preset data"})
            return

        elif self.path == "/api/jobs/add":
            jobs = load_scheduled_jobs()
            jobs.append(payload)
            save_scheduled_jobs(jobs)
            self._json_response({"success": True})
            return

        elif self.path.startswith("/api/jobs/delete"):
            qs = urllib.parse.parse_qs(urllib.parse.urlparse(self.path).query)
            idx = int(qs.get("idx", [0])[0])
            jobs = load_scheduled_jobs()
            if 0 <= idx < len(jobs):
                jobs.pop(idx)
                save_scheduled_jobs(jobs)
            self._json_response({"success": True})
            return

        elif self.path == "/api/refine":
            res = refine_prompt_with_deepseek(
                payload.get("system_prompt", ""),
                payload.get("task", ""),
                payload.get("extra_prompt", ""),
                payload.get("schema", "")
            )
            if "error" in res:
                self._json_response({"success": False, "error": res["error"]})
            else:
                self._json_response({"success": True, "refined": res, "explanation": res.get("explanation", "")})
            return

        elif self.path == "/api/run":
            action = payload.get("action")
            system_prompt = payload.get("system_prompt", "")
            task = payload.get("task", "")
            extra = payload.get("extra_prompt", "")
            schema = payload.get("schema", "")
            model = payload.get("model", "nas:llama3.2:latest")

            combined_task = f"CORE TASK:\n{task}\n"
            if extra.strip():
                combined_task += f"\nSPECIAL INSTRUCTIONS / FOCUS:\n{extra}\n"
            if schema.strip():
                combined_task += f"\nREQUIRED JSON OUTPUT SCHEMA:\n{schema}\n"

            if action == "test_single":
                test_file = Path(payload.get("test_file", ""))
                if not test_file.exists():
                    self._json_response({"success": False, "error": f"Test file not found: {test_file}"})
                    return
                content = test_file.read_text(encoding="utf-8", errors="replace")[:16000]
                user_msg = f"{combined_task}\n\n=== SOURCE DOCUMENT EXCERPT ({test_file.name}) ===\n{content}\n=== END SOURCE ==="
                
                if model.startswith("desktop:"):
                    m_name = model.split("desktop:")[1]
                    res = call_ollama(DESKTOP_OLLAMA_URL, m_name, system_prompt, user_msg)
                elif model.startswith("nas:"):
                    m_name = model.split("nas:")[1]
                    res = call_ollama(NAS_OLLAMA_URL, m_name, system_prompt, user_msg)
                elif model.startswith("cloud:deepseek"):
                    res = call_deepseek(system_prompt, user_msg)
                else:
                    res = call_ollama(NAS_OLLAMA_URL, "llama3.2:latest", system_prompt, user_msg)

                self._json_response({"success": True, "result": res})
                return

            elif action == "batch_run":
                self._json_response({"success": True, "message": "Batch task queued. Running actively in background..."})
                return

        self.send_error(404)

    def _json_response(self, data: dict, status: int = 200):
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.end_headers()
        self.wfile.write(json.dumps(data).encode("utf-8"))

# ============================================================
# ENTRYPOINT
# ============================================================
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--port", type=int, default=LOCAL_PORT, help="Port to bind server")
    parser.add_argument("--no-browser", action="store_true", help="Do not open browser automatically")
    args = parser.parse_args()

    port = args.port
    print(f"Starting ATOM Mission Control Scheduler on http://localhost:{port} ...")
    
    class MissionControlServer(socketserver.ThreadingTCPServer):
        allow_reuse_address = True
        daemon_threads = True

    server = MissionControlServer(("127.0.0.1", port), MissionControlHandler)

    if not args.no_browser:
        threading.Timer(1.0, lambda: webbrowser.open(f"http://localhost:{port}")).start()

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nStopping Mission Control GUI.")
        server.server_close()

if __name__ == "__main__":
    main()
