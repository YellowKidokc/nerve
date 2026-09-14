param([string]$CloudUrl = 'https://nerve-atom-jobs.davidokc28.workers.dev')
$ErrorActionPreference = 'Stop'
$credentialPath = Join-Path $env:LOCALAPPDATA 'NerveAtomCloud\token.xml'
if (!(Test-Path -LiteralPath $credentialPath)) { throw 'Cloud credential has not been configured.' }
$secureToken = Import-Clixml -LiteralPath $credentialPath
$tokenPointer = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($secureToken)
try {
    $env:NERVE_ATOM_TOKEN = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($tokenPointer)
    $env:NERVE_ATOM_URL = $CloudUrl
    Start-Process -FilePath pythonw -ArgumentList ('"' + (Join-Path $PSScriptRoot 'bridge.py') + '"') -WindowStyle Hidden
} finally {
    [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($tokenPointer)
    Remove-Item Env:\NERVE_ATOM_TOKEN
    Remove-Item Env:\NERVE_ATOM_URL
}
