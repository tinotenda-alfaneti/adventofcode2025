# Update Benchmark Reports for GitHub Pages
# Run this script after running: cargo bench

Write-Host "Updating benchmark reports..." -ForegroundColor Green

# Remove old docs/criterion folder
if (Test-Path "..\docs\criterion") {
    Remove-Item -Recurse -Force "..\docs\criterion"
    Write-Host "✓ Cleaned old reports" -ForegroundColor Yellow
}

# Copy new criterion reports
if (Test-Path "..\target\criterion") {
    Copy-Item -Recurse "..\target\criterion" "..\docs\criterion"
    Write-Host "Copied new reports to docs/" -ForegroundColor Green
    
    # Parse benchmark times from Criterion reports
    Write-Host "Parsing benchmark times..." -ForegroundColor Green
    $times = @{}
    
    for ($i = 1; $i -le 12; $i++) {
        $dayName = "day_{0:d2}" -f $i
        $estimatesFile = "..\target\criterion\$dayName\base\estimates.json"
        
        if (Test-Path $estimatesFile) {
            $json = Get-Content $estimatesFile | ConvertFrom-Json
            $meanNs = $json.mean.point_estimate
            
            # Convert to appropriate unit
            if ($meanNs -lt 1000) {
                $timeStr = "{0:N0} ns" -f $meanNs
            } elseif ($meanNs -lt 1000000) {
                $timeStr = "{0:N1} µs" -f ($meanNs / 1000)
            } elseif ($meanNs -lt 1000000000) {
                $timeStr = "{0:N1} ms" -f ($meanNs / 1000000)
            } else {
                $timeStr = "{0:N2} s" -f ($meanNs / 1000000000)
            }
            
            $times[$i] = $timeStr
        }
    }
    
    # Update index.html with new times
    if ($times.Count -gt 0) {
        $indexPath = "..\docs\index.html"
        if (Test-Path $indexPath) {
            $html = Get-Content $indexPath -Raw
            
            foreach ($day in $times.Keys) {
                # Update the time in the card for this day
                $pattern = "(<div class=`"card`">[\s\S]*?<a href=`"criterion/day_{0:d2}/report/index.html`">Day {1}</a>[\s\S]*?<p style=`"color: #999; margin-top: 10px;`">)[^<]+(</p>)" -f $day, $day
                $replacement = "`${1}~$($times[$day])`${2}"
                $html = $html -replace $pattern, $replacement
            }
            
            $html | Set-Content $indexPath -NoNewline
            Write-Host "✓ Updated index.html with latest times" -ForegroundColor Green
        }
    }
    
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
