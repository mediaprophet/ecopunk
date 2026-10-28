import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location("serial_target_candidate", Path(__file__).with_name("serial_check.py"))
serial = importlib.util.module_from_spec(spec)
spec.loader.exec_module(serial)

class TargetPolicyTests(unittest.TestCase):
    poet = "C:/github/poet"
    qualia = "C:/github/qualiaDB"
    base = ["cargo", "test", "--locked", "--offline", "-p", "poet-cli", "--features", "native-graphical", "--lib", "--target-dir", "C:/github/ecopunk/target"]

    def refused(self, command, cwd=None):
        with self.assertRaises(ValueError): serial.check_command(command, cwd or self.poet)

    def test_standard_poet_check_and_build(self):
        serial.check_command(self.base + ["rig_", "--", "--test-threads=1"], self.poet)
        cmd = ["cargo", "build", "--locked", "--offline", "--package=poet-cli", "--features=native-graphical", "--bin", "poet", "--target-dir=C:/github/ecopunk/target"]
        serial.check_command(cmd, self.poet)
        serial.check_command(self.base + ["--manifest-path", "C:/github/poet/Cargo.toml"], self.poet)
        serial.check_command(self.base + ["--manifest-path=Cargo.toml"], "C:/GITHUB/poet/.")

    def test_other_workspaces_and_packages_refused_clearly(self):
        with self.assertRaisesRegex(ValueError, "Qualia checks require a separate guarded isolated-target workflow"):
            serial.check_command(self.base, self.qualia)
        self.refused([x if x != "poet-cli" else "qualia-mcp" for x in self.base])
        self.refused([x if x != "poet-cli" else "qualia-core-db" for x in self.base])
        self.refused(self.base + ["--workspace"])
        with self.assertRaisesRegex(ValueError, "only accepts C:/github/poet/Cargo.toml"):
            serial.check_command(self.base + ["--manifest-path=C:/github/qualiaDB/Cargo.toml"], self.poet)

    def test_feature_and_target_overrides_refused(self):
        self.refused(self.base + ["--features", "other"])
        self.refused(self.base + ["--all-features"])
        self.refused(self.base + ["--no-default-features"])
        self.refused(self.base + ["-Fother"])
        self.refused(self.base + ["--config", "anything"])
        self.refused(self.base + ["--target-dir", "C:/github/ecopunk/target"])
        with self.assertRaisesRegex(ValueError, "other targets require a separate guarded workflow"):
            serial.check_command(self.base[:-1] + ["C:/github/other-target"], self.poet)

    def test_path_normalization_and_option_validation(self):
        serial.check_command(self.base[:-1] + ["c:/GITHUB/ecopunk/target/."], "C:/GITHUB/poet/.")
        self.refused(self.base + ["--manifest-path", "Cargo.toml", "--manifest-path", "Cargo.toml"])
        with self.assertRaisesRegex(ValueError, "shared target requires -p poet-cli"):
            serial.check_command([x if x != "poet-cli" else "qualia-mcp" for x in self.base], self.poet)
        self.refused([x for x in self.base if x != "--locked"])
        self.refused([x for x in self.base if x != "--offline"])
        self.refused([x for x in self.base if x not in ("-p", "poet-cli")])
        self.refused([x for x in self.base if x not in ("--features", "native-graphical")])
        self.refused(["cargo", "clean", "--locked", "--offline", "--target-dir", "C:/github/ecopunk/target"])

if __name__ == "__main__": unittest.main()
