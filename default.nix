{ stdenv, fetchFromGitHub, rustPlatform, perl }:

rustPlatform.buildRustPackage rec {
  name = "tw-rs-${version}";
  version = "0.1.23";

  src = fetchFromGitHub {
    owner = "vmchale";
    repo = "tw-rs";
    rev = "${version}";
    sha256 = "0090p7p9gyv688w9vx2vr976vk2qfg8yc8aiv6mwvqi5a5l9kcv5";
  };
  buildInputs = [ perl ];

  depsSha256 = "1wcq8d6jg49rkmxcl9gk3c3n58hpia7id3iv2mqj09kw5a6cxl1y";

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
