# scripts/grok-windows-artifact.ps1
param([Parameter(Mandatory=$true)][string]$DestDir)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Path $DestDir -Force | Out-Null
try {
  $ver = (Invoke-WebRequest -Uri 'https://x.ai/cli/stable' -UseBasicParsing).Content.Trim()
  if ($ver -notmatch '^\d+\.\d+\.\d+') { throw "bad version $ver" }
  $url = "https://x.ai/cli/grok-$ver-windows-x86_64.exe"
  $out = Join-Path $DestDir 'grok.exe'
  Invoke-WebRequest -Uri $url -OutFile $out -UseBasicParsing
  Copy-Item $out (Join-Path $DestDir 'agent.exe')
  Write-Output "bundled $ver"
  exit 0
} catch {
  Write-Warning "grok download skipped: $_"
  exit 0
}
