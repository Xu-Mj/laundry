param (
    [string]$VersionType = "patch"  # Default to updating patch version
)

# Define file paths
$cargoTomlPath = "src-tauri/Cargo.toml"
$tauriConfPath = "src-tauri/tauri.conf.json"
$packageJsonPath = "package.json"

# Function to increment version with decimal-based strategy
function Increment-Version {
    param (
        [string]$Version,
        [string]$Type
    )
    
    $versionParts = $Version.Split('.')
    
    switch ($Type) {
        "major" {
            $versionParts[0] = [int]$versionParts[0] + 1
            $versionParts[1] = 0
            $versionParts[2] = 0
        }
        "minor" {
            # If minor is 9, increment major and reset minor
            if ([int]$versionParts[1] -eq 9) {
                $versionParts[0] = [int]$versionParts[0] + 1
                $versionParts[1] = 0
                $versionParts[2] = 0
            } else {
                $versionParts[1] = [int]$versionParts[1] + 1
                $versionParts[2] = 0
            }
        }
        "patch" {
            # If patch is 9, increment minor and reset patch
            if ([int]$versionParts[2] -eq 9) {
                # If minor is also 9, increment major
                if ([int]$versionParts[1] -eq 9) {
                    $versionParts[0] = [int]$versionParts[0] + 1
                    $versionParts[1] = 0
                    $versionParts[2] = 0
                } else {
                    $versionParts[1] = [int]$versionParts[1] + 1
                    $versionParts[2] = 0
                }
            } else {
                $versionParts[2] = [int]$versionParts[2] + 1
            }
        }
    }
    
    return $versionParts -join '.'
}

# Function to check if git is available
function Test-GitAvailable {
    try {
        $null = git --version
        return $true
    } catch {
        return $false
    }
}

# Function to check if current directory is a git repository
function Test-GitRepository {
    try {
        $null = git rev-parse --is-inside-work-tree
        return $true
    } catch {
        return $false
    }
}

# Validate version type input
if ($VersionType -notin @("major", "minor", "patch")) {
    Write-Error "Invalid version type. Must be one of: major, minor, patch"
    exit 1
}

# Get current version from package.json
if (Test-Path $packageJsonPath) {
    $packageJson = Get-Content $packageJsonPath -Raw | ConvertFrom-Json
    $currentVersion = $packageJson.version
    Write-Host "Current version: $currentVersion"
    
    # Calculate new version
    $newVersion = Increment-Version -Version $currentVersion -Type $VersionType
    Write-Host "New version: $newVersion"
    
    # Update package.json
    $packageJson.version = $newVersion
    $packageJson | ConvertTo-Json -Depth 100 | Set-Content $packageJsonPath
    Write-Host "Updated $packageJsonPath"
    
    # Update Cargo.toml
    if (Test-Path $cargoTomlPath) {
        $cargoToml = Get-Content $cargoTomlPath -Raw
        
        # Only update the version in [package] section
        $packageSectionPattern = "(?s)(\[package\][^\[]*?version\s*=\s*"")([0-9]+\.[0-9]+\.[0-9]+)("")"
        $cargoToml = [regex]::Replace($cargoToml, $packageSectionPattern, "`${1}$newVersion`${3}")
        
        Set-Content -Path $cargoTomlPath -Value $cargoToml
        Write-Host "Updated $cargoTomlPath"
    } else {
        Write-Warning "Could not find $cargoTomlPath"
    }
    
    # Update tauri.conf.json
    if (Test-Path $tauriConfPath) {
        $tauriConf = Get-Content $tauriConfPath -Raw | ConvertFrom-Json
        $tauriConf.version = $newVersion
        $tauriConf | ConvertTo-Json -Depth 100 | Set-Content $tauriConfPath
        Write-Host "Updated $tauriConfPath"
    } else {
        Write-Warning "Could not find $tauriConfPath"
    }
    
    Write-Host "Version updated to $newVersion successfully!"
    
    # Git commit if available
    if (Test-GitAvailable) {
        if (Test-GitRepository) {
            Write-Host "Committing changes to Git..."
            
            # Add changed files
            git add $packageJsonPath
            if (Test-Path $cargoTomlPath) {
                git add $cargoTomlPath
            }
            if (Test-Path $tauriConfPath) {
                git add $tauriConfPath
            }
            
            # Commit with version update message
            git commit -m "feat: update version number to $newVersion"
            
            Write-Host "Changes committed to Git with message: 'feat: update version number to $newVersion'"
        } else {
            Write-Warning "Current directory is not a Git repository. Skipping Git commit."
        }
    } else {
        Write-Warning "Git is not available. Skipping Git commit."
    }
} else {
    Write-Error "Could not find $packageJsonPath"
    exit 1
} 