{ stdenv, fetchFromGitHub, rustPlatform, perl, zlib, openssl }:

rustPlatform.buildRustPackage rec {
  name = "tw-rs-${version}";
  version = "0.1.24";

  src = fetchFromGitHub {
    owner = "vmchale";
    repo = "tw-rs";
    rev = "${version}";
    sha256 = "11bbla9r1q9hlr4xb0n87x9yf63jz8qx09i3yjy2p8jfdym7v81v";
  };
  buildInputs = [ perl zlib openssl ];

  depsSha256 = "040i3nks8mgzhz98gcl83pbpfsvbbynvqifzq0rcdba5chi2xnmv";

  installPhase = ''
    mkdir -p $out/bin
    cp -p target/release/tokei $out/bin/
  '';

  meta = with stdenv.lib; {
    description = "Twitter command-line interface written in rust";
    homepage = https://github.com/vmchale/tw-rs;
    license = licenses.bsd3;
    #maintainers = with maintainers; [ gebner ];
    platforms = platforms.all;
  };
}
