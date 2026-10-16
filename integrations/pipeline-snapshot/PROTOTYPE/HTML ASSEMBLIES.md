# HTML assembly stations

Seven API groups now have explicit HTML assembly companions in the prototype:

| API group | Assembly identity | Initial layout direction |
|---|---|---|
| 01_CKG | 01A_CKG_HTML | Shared report shell |
| 02_C_P_E_L | 02A_C_P_E_L_HTML | Distinct C_P_E_L layout |
| 02_YOUTUBE | 02A_YOUTUBE_HTML | Shared report shell |
| 04_ACADEMIA | 04A_ACADEMIA_HTML | Distinct Academia layout |
| 06_ATOMS | 06A_ATOMS_HTML | Distinct Atoms layout |
| 07_ARGUMENTS | 07A_ARGUMENTS_HTML | Shared report shell |
| 08_API_DEEP | 08A_API_DEEP_HTML | Distinct API Deep layout |

These identities are proposed configuration names, not new folders or stations created in pipeline-workflows. The two groups numbered 02 remain distinct by full identity. C_P_E_L exists in David's supplied list but not in the inspected cloud checkout.

The shared shell for CKG, YouTube, and Arguments is an editable starting assumption. The four distinct groups were explicitly identified by David; their detailed schemas and exact layouts still need to be supplied. All layouts can share rendering utilities while using separate templates and field mappings.

The HTML Assemblies screen allows selecting one of the seven stations, editing its fixed output folder, template identity, and ordered display sections, then downloading its design definition. All seven group/assembly pairs appear in the sample workflow. Adding a designated group adds its assembly. Assemblies moved before their source group show a warning; this is not an execution dependency engine.

Read completed JSON records, validate against the group's contract, then render HTML. Do not assume every JSON file is a final answer: ignore incomplete or failed artifacts, and retain record IDs, versions, source references, and validation receipts. One source note may produce many records that must be reconciled before assembling that note's HTML. Independent finished notes can render and save immediately. A later amendment creates a new artifact version and invalidates the prior version's review for the amended content.

The intended implementation is Python-based HTML rendering, not browser WebAssembly. Rendering itself does not require another model call. Exact source JSON mappings are pending; changing section names in the prototype does not implement those mappings.

A completed assembly enters HTML Review. Processing can continue while review is pending, unless the workflow explicitly selects waiting for approval. Passing review does not itself publish to a website. Website destination and publication behavior remain unspecified.

This update is a design prototype. No API, rendering service, filesystem watcher, real JSON ingestion, or publishing integration was added.
