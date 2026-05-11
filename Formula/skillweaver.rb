class Skillweaver < Formula
  desc "TUI package manager for AI agent skills and rules"
  homepage "https://github.com/GustavoGutierrez/skillweaver"
  url "https://github.com/GustavoGutierrez/skillweaver/releases/download/v0.0.1/skillweaver-x86_64-unknown-linux-gnu.tar.gz"
  sha256 "6e00a4c086ad30e7cd2b9382c2181759f003351f30d5b8e18b041f11b8057081"
  version "0.0.1"
  license "MIT"

  on_linux do
    def install
      bin.install "skillweaver"
    end
  end
end
