# Update Benchmark Reports for GitHub Pages
# Run this script after running: cargo bench

Write-Host "pdating benchmark reports..." -ForegroundColor Green

# Remove old docs/criterion folder
if (Test-Path "..\docs\criterion") {
    Remove-Item -Recurse -Force "..\docs\criterion"
    Write-Host "Cleaned old reports" -ForegroundColor Yellow
}

# Copy new criterion reports
if (Test-Path "..\target\criterion") {
    Copy-Item -Recurse "..\target\criterion" "..\docs\criterion"
    Write-Host "Copied new reports to docs/" -ForegroundColor Green
    Write-Host ""
    Write-Host "Reports updated successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "  1. Open docs/index.html in your browser to preview"
    Write-Host "  2. Commit and push to GitHub"
    Write-Host "  3. Enable GitHub Pages in repo settings (source: docs folder)"
    Write-Host ""
} else {
    Write-Host "No criterion reports found in target/criterion" -ForegroundColor Red
    Write-Host "   Run 'cargo bench' first!" -ForegroundColor Yellow
}
