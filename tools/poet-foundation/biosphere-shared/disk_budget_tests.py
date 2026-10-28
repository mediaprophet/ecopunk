"""Maintenance checks; no Cargo processes or large files are created."""
import unittest
from types import SimpleNamespace
from unittest.mock import Mock, patch

import disk_budget as budget


class DiskBudgetTests(unittest.TestCase):
    def test_start_refuses_low_space_and_accepts_exact_boundary(self):
        with patch.object(budget.shutil, "disk_usage", return_value=SimpleNamespace(free=budget.MIN_START_FREE - 1)):
            with self.assertRaisesRegex(RuntimeError, "Cargo not started"):
                budget.require_start_space("unused")
        with patch.object(budget.shutil, "disk_usage", return_value=SimpleNamespace(free=budget.MIN_START_FREE)):
            self.assertEqual(budget.require_start_space("unused"), budget.MIN_START_FREE)

    def test_running_reserve_stops_only_supplied_child_and_returns_failure(self):
        child = Mock()
        child.poll.return_value = None
        with patch.object(budget.shutil, "disk_usage", return_value=SimpleNamespace(free=budget.MIN_RUNNING_FREE - 1)), \
                patch.object(budget, "stop_owned_process") as stop:
            self.assertEqual(budget.wait_with_disk_budget(child, "unused"), budget.DISK_GUARD_EXIT)
        stop.assert_called_once_with(child)

    def test_ordinary_exit_and_existing_failure_are_preserved(self):
        for result in (0, 101):
            child = Mock()
            child.poll.return_value = result
            with patch.object(budget.shutil, "disk_usage") as disk, patch.object(budget, "stop_owned_process") as stop:
                self.assertEqual(budget.wait_with_disk_budget(child, "unused"), result)
            disk.assert_not_called()
            stop.assert_not_called()


if __name__ == "__main__":
    unittest.main()
