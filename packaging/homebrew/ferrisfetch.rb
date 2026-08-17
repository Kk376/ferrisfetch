class Ferrisfetch < Formula
  desc "Fast, lightweight Linux system information fetch CLI written in Rust"
  homepage "https://github.com/kk376/ferrisfetch"
  url "https://github.com/kk376/ferrisfetch/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "6e2fbf052988cb945c436087f42dcf9ebd8885d3cd19638b58221e7d41eca9df"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
    bash_completion.install "completions/ferrisfetch.bash" => "ferrisfetch"
    zsh_completion.install "completions/_ferrisfetch"
    fish_completion.install "completions/ferrisfetch.fish"
  end

  test do
    assert_match "ferrisfetch", shell_output("#{bin}/ferrisfetch --version")
  end
end
