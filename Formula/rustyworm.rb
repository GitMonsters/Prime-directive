# Homebrew formula for RustyWorm
# SHA256 checksums are updated automatically by the release workflow.
# To install from this tap:
#   brew tap GitMonsters/Prime-directive https://github.com/GitMonsters/Prime-directive
#   brew install rustyworm
#
# Or build from source:
#   brew install --build-from-source rustyworm

class Rustyworm < Formula
  desc "Universal AI Mimicry Engine with Dual-Process Architecture"
  homepage "https://github.com/GitMonsters/Prime-directive"
  version "2.0.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/GitMonsters/Prime-directive/releases/download/v2.0.0/rustyworm-macos-arm64"
      sha256 "PLACEHOLDER_SHA256"

      def install
        bin.install "rustyworm-macos-arm64" => "rustyworm"
      end
    end

    on_intel do
      url "https://github.com/GitMonsters/Prime-directive/releases/download/v2.0.0/rustyworm-macos-amd64"
      sha256 "PLACEHOLDER_SHA256"

      def install
        bin.install "rustyworm-macos-amd64" => "rustyworm"
      end
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/GitMonsters/Prime-directive/releases/download/v2.0.0/rustyworm-linux-arm64"
      sha256 "PLACEHOLDER_SHA256"

      def install
        bin.install "rustyworm-linux-arm64" => "rustyworm"
      end
    end

    on_intel do
      url "https://github.com/GitMonsters/Prime-directive/releases/download/v2.0.0/rustyworm-linux-amd64"
      sha256 "PLACEHOLDER_SHA256"

      def install
        bin.install "rustyworm-linux-amd64" => "rustyworm"
      end
    end
  end

  head do
    url "https://github.com/GitMonsters/Prime-directive.git", branch: "main"
    depends_on "rust" => :build

    def install
      system "cargo", "build", "--release", "--features", "api", "--bin", "rustyworm"
      bin.install "target/release/rustyworm"
    end
  end

  test do
    assert_match "RustyWorm", shell_output("#{bin}/rustyworm --help 2>&1", 0)
  end
end
