# Copyright (c) 2022 Tony Barbitta
# 
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

$this_dir = $pwd

echo "Getting output directory..."
$out_dir = (Join-Path "$this_dir" "ext" "out")
echo "Out Dir: $out_dir"
if (!(Test-Path "$out_dir")) {
    echo "Making output directory..."
    mkdir -p $out_dir >> $null
}
if (Test-Path "$out_dir\*") {
    echo "Clearing output directory..."
    rm -r -for "$out_dir\*"
}

echo "Getting input directory..."
$input_dir = (Join-Path "$this_dir" "ext" "src")
echo "Input Dir: $input_dir"
if (!(Test-Path "$input_dir")) {
    echo "Input directory does not exist!"
    $this_dir = $null
    $out_dir = $null
    $input_dir = $null
    return
}
if (!(Test-Path "$input_dir\*")) {
    echo "No files found in input directory!"
    $this_dir = $null
    $out_dir = $null
    $input_dir = $null
    return
}

echo "Getting input files..."
$in_files = (Get-ChildItem -Path $input_dir -Filter '*.rs')
$total_in_files = $in_files.Count
echo "Found $total_in_files files to compile."

$_i = 0
$in_files | % {
    $_i += 1
    echo "Compiling file $_i/$total_in_files - $($_.Name)"
    $output_dir = (Join-Path "$out_dir" $_.BaseName)
    if (!(Test-Path $output_dir)) {
        mkdir $output_dir >> $null
    }
    rustc --crate-type cdylib --out-dir $output_dir $_
    if (!(Test-Path "$output_dir\*")) {
        echo "No output seems to be generated for $($_.BaseName)"
    }
}

$this_dir = $null
$out_dir = $null
$input_dir = $null
$in_files = $null
$total_in_files = $null
$_i = $null