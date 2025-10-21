# WebTOTP 启动脚本
# 用法: .\run.ps1 [master-key]
# 示例: .\run.ps1 "MySecurePassword123"

param(
    [string]$MasterKey = ""
)

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║                    WebTOTP Server                         ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "🌐 Access the application at:" -ForegroundColor Cyan
Write-Host "   http://localhost:18007" -ForegroundColor Yellow
Write-Host ""

# 检查是否提供了主密钥
if ($MasterKey -eq "") {
    Write-Host "⚠️  No master key provided as argument" -ForegroundColor Yellow
    Write-Host "Usage: .\run.ps1 `"your-master-key`"" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Starting server in interactive mode..." -ForegroundColor Cyan
    Write-Host "You will be prompted to enter the master key" -ForegroundColor Cyan
    Write-Host ""
    
    # 交互式启动
    cd d:\worktest\webtotp
    .\target\release\webtotp.exe
} else {
    # 使用环境变量启动
    Write-Host "✓ Master key provided" -ForegroundColor Green
    Write-Host "Starting server with master key from argument..." -ForegroundColor Cyan
    Write-Host ""
    
    $env:WEBTOTP_MASTER_KEY = $MasterKey
    cd d:\worktest\webtotp
    .\target\release\webtotp.exe
}

