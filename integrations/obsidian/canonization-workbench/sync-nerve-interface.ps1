$ErrorActionPreference = "Stop"

$repoRelativeSource = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot "..\..\..\source\html\atoms\atom-builder.html"))
$source = if (Test-Path -LiteralPath $repoRelativeSource -PathType Leaf) {
    $repoRelativeSource
} else {
    "D:\GitHub\nerve\source\html\atoms\atom-builder.html"
}
$destinationDirectory = Join-Path $PSScriptRoot "nerve"
$destination = Join-Path $destinationDirectory "atom-builder.html"

if (-not (Test-Path -LiteralPath $source -PathType Leaf)) {
    throw "Nerve source interface was not found: $source"
}

New-Item -ItemType Directory -Path $destinationDirectory -Force | Out-Null
Copy-Item -LiteralPath $source -Destination $destination -Force

$sourceHash = (Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash
$destinationHash = (Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash
if ($sourceHash -ne $destinationHash) {
    throw "Generated Obsidian interface does not match the Nerve source."
}

[pscustomobject]@{
    status = "SYNCED_BYTE_IDENTICAL"
    authority = $source
    projection = $destination
    sha256 = $sourceHash
    canonical_admission = $false
} | ConvertTo-Json
