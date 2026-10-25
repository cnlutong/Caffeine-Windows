#requires -Version 7.0
param([string]$MakeAppxPath = 'makeappx.exe')

$ErrorActionPreference = 'Stop'
$makeAppx = (Get-Command $MakeAppxPath -CommandType Application -ErrorAction Stop).Source
$repoRoot = Split-Path $PSScriptRoot -Parent
$projectPath = Join-Path $repoRoot 'source\Caffeine.csproj'
[xml]$project = Get-Content -LiteralPath $projectPath -Raw
[xml]$manifest = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'AppxManifest.xml') -Raw
$manifest.Package.Identity.Version = "$($project.Project.PropertyGroup.Version).0"
$version = $manifest.Package.Identity.Version
$outputRoot = [IO.Path]::GetFullPath((Join-Path $repoRoot 'artifacts\msix'))
$stage = Join-Path $outputRoot ('build-' + [guid]::NewGuid().ToString('N'))
$payload = Join-Path $stage 'payload'
$packagePath = Join-Path $outputRoot "Caffeine-Windows_${version}_x64.msix"
New-Item -ItemType Directory -Path $payload -Force | Out-Null

try {
    dotnet publish $projectPath -c Release -r win-x64 --self-contained true -o (Join-Path $payload 'App')
    if ($LASTEXITCODE -ne 0) { throw 'dotnet publish failed.' }

    $manifest.Save((Join-Path $payload 'AppxManifest.xml'))
    Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'Assets') -Destination $payload -Recurse
    Copy-Item -LiteralPath (Join-Path $repoRoot 'LICENSE') -Destination $payload
    $builtPackage = Join-Path $stage 'Caffeine.msix'
    & $makeAppx pack /d $payload /p $builtPackage /h SHA256 /o
    if ($LASTEXITCODE -ne 0) { throw 'MSIX packaging or manifest validation failed.' }

    # Check the actual package contents and run the existing self-test from the extracted package.
    $unpacked = Join-Path $stage 'verified'
    & $makeAppx unpack /p $builtPackage /d $unpacked /o
    if ($LASTEXITCODE -ne 0) { throw 'MSIX unpack verification failed.' }
    [xml]$packedManifest = Get-Content -LiteralPath (Join-Path $unpacked 'AppxManifest.xml') -Raw
    if ($packedManifest.Package.Identity.OuterXml -ne $manifest.Package.Identity.OuterXml -or
        $packedManifest.Package.Properties.PublisherDisplayName -ne 'Tong Lu') {
        throw 'Packaged identity does not match the Store identity.'
    }
    $packedExe = Join-Path $unpacked 'App\Caffeine.exe'
    if ((Get-FileHash -LiteralPath $packedExe).Hash -ne
        (Get-FileHash -LiteralPath (Join-Path $payload 'App\Caffeine.exe')).Hash) {
        throw 'Packaged executable does not match the build.'
    }
    $test = Start-Process -FilePath $packedExe -ArgumentList '--self-test' -Wait -PassThru -WindowStyle Hidden
    if ($test.ExitCode -ne 0) { throw "Packaged executable self-test failed: $($test.ExitCode)" }

    Copy-Item -LiteralPath $builtPackage -Destination $packagePath -Force
    $hash = (Get-FileHash -LiteralPath $packagePath -Algorithm SHA256).Hash
    "$hash  $([IO.Path]::GetFileName($packagePath))" | Set-Content -LiteralPath "$packagePath.sha256" -Encoding ascii
    Write-Host "Store upload package (unsigned): $packagePath"
}
finally {
    $resolvedStage = [IO.Path]::GetFullPath($stage)
    if (-not $resolvedStage.StartsWith($outputRoot + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) {
        throw 'Refusing to remove a staging directory outside the MSIX output directory.'
    }
    Remove-Item -LiteralPath $resolvedStage -Recurse -Force
}
