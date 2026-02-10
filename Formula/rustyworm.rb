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
      sha256 "8785f6b7ed28bbe11389befc3576c6ff3539d2eed56347d4a8c8dd52e8d32f0d"

      def install
        bin.install "rustyworm-macos-arm64" => "rustyworm"
      end
    end

    on_intel do
      url "https://github.com/GitMonsters/Prime-directive/releases/download/v2.0.0/rustyworm-macos-amd64"
      sha256 "25ef0f68b4280c78ace598942510507378e02d3f2998b1fe2f4f1fe43636af11"

      def install
        bin.install "rustyworm-macos-amd64" => "rustyworm"
      end
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/GitMonsters/Prime-directive/releases/download/v2.0.0/rustyworm-linux-arm64"
      sha256 "2037548e42dec7969c6f6040a2286db8b556201cf3336b9a93f1f706d49104cc"

      def install
        bin.install "rustyworm-linux-arm64" => "rustyworm"
      end
    end

    on_intel do
      url "https://github.com/GitMonsters/Prime-directive/releases/download/v2.0.0/rustyworm-linux-amd64"
      sha256 "2b519ada372f9d8745f9a7b7df26ea56f72d8721ff4dd5f77cfa0746485f278a"

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
