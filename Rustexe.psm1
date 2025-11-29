function Rustexe($Options) {
    $path = Join-Path $PSScriptRoot 'target' 'debug' 'Rustexe.exe'
    & "$path"
}

Export-ModuleMember -Function Rustexe