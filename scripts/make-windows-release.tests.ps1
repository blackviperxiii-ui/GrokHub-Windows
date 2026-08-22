# scripts/make-windows-release.tests.ps1
$src = Get-Content -Raw "$PSScriptRoot/make-windows-release.ps1"
if ($src -notmatch 'missing grokhub.exe') { throw 'pack script must fail closed without grokhub.exe' }
if ($src -notmatch 'ProgramFiles\(x86\)') { throw 'pack script must probe x86 Inno Setup' }
if ($src -notmatch 'Get-Command ISCC') { throw 'pack script must probe PATH ISCC' }
if ($src -notmatch 'missing GrokHub-Setup-\$Ver.exe') { throw 'pack script must fail if Setup.exe is missing after ISCC' }
if ($src -notmatch 'grok-windows-artifact.ps1') { throw 'pack script must call grok download helper' }
$dl = Get-Content -Raw "$PSScriptRoot/grok-windows-artifact.ps1"
if ($dl -notmatch 'exit 0') { throw 'grok download failure must not fail the cabin pack' }
if ($dl -notmatch 'x.ai/cli/stable') { throw 'must resolve version from x.ai/cli/stable' }
Write-Output 'pack script locks ok'
