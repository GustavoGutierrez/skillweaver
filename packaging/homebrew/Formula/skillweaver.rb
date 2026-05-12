class Skillweaver < Formula
  desc "TUI package manager for AI agent skills and rules"
  homepage "https://github.com/GustavoGutierrez/skillweaver"
  url "https://github.com/GustavoGutierrez/skillweaver/releases/download/{{VERSION}}/skillweaver-x86_64-unknown-linux-gnu.tar.gz"
  sha256 "{{CHECKSUM}}"
  version "{{VERSION}}"
  license "MIT"

  on_linux do
    def install
      bin.install "skillweaver"
    end
  end
end