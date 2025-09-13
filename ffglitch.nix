{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation rec {
  pname = "ffglitch";
  version = "0.10.2";
  
  src = pkgs.fetchzip {
    url = "https://ffglitch.org/pub/bin/macos-aarch64/ffglitch-${version}-macos-aarch64.zip";
    sha256 = "sha256-ypmKBwLGEIwqQw9Zq/FjQeorROSW0BmQIHb5CkymWuk=";
    stripRoot = false;
  };
  
  nativeBuildInputs = with pkgs; [ python3 ];
  
  dontBuild = true;
  
  installPhase = ''
    mkdir -p $out/bin
    
    # Debug: Zeige alle Dateien
    echo "=== All files in ZIP ==="
    find . -type f -ls
    echo "======================="
    
    # Kopiere von dem richtigen Unterordner
    cp -v ./ffglitch-*/ffedit ./ffglitch-*/ffgac ./ffglitch-*/fflive ./ffglitch-*/qjs $out/bin/ || true
    
    # Kopiere Python scripts falls vorhanden
    find ./ffglitch-* -name "*.py" -type f -exec cp -v {} $out/bin/ \; || true
    
    # Kopiere readme für später
    find ./ffglitch-* -name "readme.txt" -type f -exec cp -v {} $out/bin/ \; || true
    
    # Erstelle ffglitch Alias zu ffedit (das ist wahrscheinlich das Haupttool)
    ln -s ffedit $out/bin/ffglitch || true
  '';
  
  meta = with pkgs.lib; {
    description = "FFmpeg-based multimedia bitstream editor for glitch art";
    homepage = "https://ffglitch.org/";
    platforms = pkgs.lib.platforms.darwin;
  };
}
