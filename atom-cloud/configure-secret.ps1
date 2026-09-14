$ErrorActionPreference = 'Stop'
$credentialDirectory = Join-Path $env:LOCALAPPDATA 'NerveAtomCloud'
$credentialPath = Join-Path $credentialDirectory 'token.xml'
if (Test-Path -LiteralPath $credentialPath) { throw 'A credential already exists. Refusing to replace it.' }
New-Item -ItemType Directory -Path $credentialDirectory -Force | Out-Null
$tokenBytes = New-Object byte[] 32
$rng = [Security.Cryptography.RandomNumberGenerator]::Create()
$rng.GetBytes($tokenBytes)
$rng.Dispose()
$serviceToken = [Convert]::ToBase64String($tokenBytes)
# PowerShell protects this credential using Windows DPAPI for the current user.
ConvertTo-SecureString -String $serviceToken -AsPlainText -Force | Export-Clixml -LiteralPath $credentialPath
$serviceToken | & wrangler secret put NERVE_ATOM_TOKEN --config (Join-Path $PSScriptRoot 'wrangler.jsonc')
if ($LASTEXITCODE -ne 0) { throw 'Secret upload failed. The encrypted local credential was preserved for recovery.' }
$serviceToken = $null
