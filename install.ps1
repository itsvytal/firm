$ErrorActionPreference = "Stop"

# Configuration
$Name = "firm"
$GitHubRepo = "42futures/firm"

# Detect architecture
$Arch = (Get-ComputerInfo).CsSystemType.ToLower()
if ($Arch.StartsWith("x64") -or $Arch.StartsWith("amd64")) {
    $Archive = "firm-windows-amd64"
} else {
    Write-Host "Error: Unsupported architecture: $Arch" -ForegroundColor Red
    exit 1
}

$Url = "https://github.com/$GitHubRepo/releases/latest/download/$Archive.zip"

# Download archive
Write-Host "Downloading $Name..."
$TempArchive = "$env:TEMP\$Archive.zip"
Invoke-WebRequest -Uri $Url -OutFile $TempArchive

# Extract archive
$TempExtract = "$env:TEMP\$Archive"
Expand-Archive -Path $TempArchive -DestinationPath $TempExtract -Force

# Install to user's local bin
$InstallDir = "$env:LOCALAPPDATA\Programs\$Name"
New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
Move-Item -Path "$TempExtract\$Name.exe" -Destination "$InstallDir\$Name.exe" -Force

# Add to PATH if not already present
$UserPath = [Environment]::GetEnvironmentVariable('Path', 'User')
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable('Path', "$UserPath;$InstallDir", 'User')
    Write-Host "Added $InstallDir to PATH (restart shell to use)"
}

# Cleanup
Remove-Item -Path $TempArchive -Force
Remove-Item -Path $TempExtract -Recurse -Force

Write-Host "✓ Installed to $InstallDir\$Name.exe" -ForegroundColor Green
