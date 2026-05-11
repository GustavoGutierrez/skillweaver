class Skillweaver < Formula
  desc "TUI package manager for AI agent skills and rules"
  homepage "https://github.com/GustavoGutierrez/skillweaver"
  url "https://github.com/GustavoGutierrez/skillweaver/releases/download/v0.0.1/skillweaver-x86_64-unknown-linux-gnu.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256_AFTER_FIRST_RELEASE"
  version "0.0.1"
  license "MIT"

  on_linux do
    def install
      bin.install "skillweaver"
    end
  end
end
