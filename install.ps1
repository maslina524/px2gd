$install_folder = "$env:APPDATA\px2gd"
$install_path = "$install_folder\px2gd.exe"

Write-Host "Download to '$install_folder'" -ForegroundColor Cyan

if (-not (Test-Path $install_folder)) {
    New-Item -Path $install_folder -ItemType Directory -Force | Out-Null
}

if (-not (Test-Path $install_path)) {
    try {
        Invoke-WebRequest -Uri "https://github.com/maslina524/px2gd/releases/download/1.0.1/px2gd.exe" -OutFile $install_path
    }
    catch {
        Write-Host "Download error: $($_.Exception.Message)" -ForegroundColor Red
        exit 1
    }
    Write-Host "The Px2Gd has been successfully installed!" -ForegroundColor Green
} else {
    Write-Host "Px2Gd is already installed" -ForegroundColor Yellow
}
Write-Host ""

Write-Host "Installing Px2Gd to the PATH" -ForegroundColor Cyan
$path = [Environment]::GetEnvironmentVariable("Path", "User")
$paths = $path -split ';' | Where-Object { $_ -ne '' }
if ($paths -contains $install_folder) {
    Write-Host "Px2Gd is already installed in the PATH" -ForegroundColor Yellow
} else {
    [Environment]::SetEnvironmentVariable(
        "Path",
        [Environment]::GetEnvironmentVariable("Path", "User") + ";$install_folder",
        "User"
    )
    Write-Host "Px2Gd has been successfully installed in the PATH!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Restart the terminal and you will be able to use Px2Gd" -ForegroundColor Yellow
}
