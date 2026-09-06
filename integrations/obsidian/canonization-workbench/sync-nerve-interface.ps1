$ErrorActionPreference = "Stop"

$repoRelativeSource = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot "..\..\..\source\html\atoms\atom-builder.html"))
$source = if (Test-Path -LiteralPath $repoRelativeSource -PathType Leaf) {
    $repoRelativeSource
} else {
    "D:\GitHub\nerve\source\html\atoms\atom-builder.html"
}
$destinationDirectory = Join-Path $PSScriptRoot "nerve"
$destination = Join-Path $destinationDirectory "atom-builder.html"
$dependencyNames = @("field-registry.js", "prompt-rail.js", "prompt-rail.css", "prompt-target.css", "workbench.html")

if (-not (Test-Path -LiteralPath $source -PathType Leaf)) {
    throw "Nerve source interface was not found: $source"
}

New-Item -ItemType Directory -Path $destinationDirectory -Force | Out-Null
Copy-Item -LiteralPath $source -Destination $destination -Force

$sourceDirectory = Split-Path -Parent $source
foreach ($dependencyName in $dependencyNames) {
    $dependencySource = Join-Path $sourceDirectory $dependencyName
    if (-not (Test-Path -LiteralPath $dependencySource -PathType Leaf)) {
        throw "Nerve interface dependency was not found: $dependencySource"
    }
    Copy-Item -LiteralPath $dependencySource -Destination (Join-Path $destinationDirectory $dependencyName) -Force
}

$sourceHash = (Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash
$destinationHash = (Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash
if ($sourceHash -ne $destinationHash) {
    throw "Generated Obsidian interface does not match the Nerve source."
}

$dependencyHashes = @{}
foreach ($dependencyName in $dependencyNames) {
    $dependencySource = Join-Path $sourceDirectory $dependencyName
    $dependencyDestination = Join-Path $destinationDirectory $dependencyName
    $sourceDependencyHash = (Get-FileHash -LiteralPath $dependencySource -Algorithm SHA256).Hash
    $destinationDependencyHash = (Get-FileHash -LiteralPath $dependencyDestination -Algorithm SHA256).Hash
    if ($sourceDependencyHash -ne $destinationDependencyHash) {
        throw "Generated Obsidian dependency does not match the Nerve source: $dependencyName"
    }
    $dependencyHashes[$dependencyName] = $sourceDependencyHash
}

[pscustomobject]@{
    status = "SYNCED_BYTE_IDENTICAL"
    authority = $source
    projection = $destination
    sha256 = $sourceHash
    dependencies = $dependencyHashes
    canonical_admission = $false
} | ConvertTo-Json
