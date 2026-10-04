import pathlib
import tempfile
import unittest

from import_mapflags import import_manifest, parse_line


class MapFlagImportTests(unittest.TestCase):
    def test_structured_flags_keep_their_primary_arguments(self):
        flags = {"nosave", "pvp_nightmaredrop", "skill_damage", "skill_duration", "battleground"}
        skills = {"HT_SKIDTRAP": 115}
        self.assertEqual(parse_line("test mapflag nosave prontera,155,181", skills, flags)["save"], ["prontera", 155, 181])
        self.assertEqual(parse_line("test mapflag pvp_nightmaredrop random,equip,300", skills, flags)["arguments"], [-1, 2, 300])
        self.assertEqual(parse_line("test mapflag skill_damage HT_SKIDTRAP,BL_PC,-100,50", skills, flags)["arguments"], [115, 1, -100, 50, 0, 0])
        self.assertEqual(parse_line("test mapflag skill_duration HT_SKIDTRAP,400", skills, flags)["arguments"], [115, 400])
        self.assertEqual(parse_line("test mapflag battleground", skills, flags)["arguments"], [1])
        self.assertFalse(parse_line("test mapflag battleground off", skills, flags)["enabled"])

    def test_enabled_shared_sources_precede_pre_renewal_overrides_and_ignore_comments(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / "npc/pre-re").mkdir(parents=True)
            (root / "npc/scripts_mapflags.conf").write_text("npc: npc/shared.txt\n//npc: npc/inactive.txt\n")
            (root / "npc/pre-re/scripts_mapflags.conf").write_text("npc: npc/pre-re/classic.txt\n")
            (root / "npc/shared.txt").write_text("/* fake mapflag gvg */\nTown.gat mapflag pvp\n//Ignored mapflag noskill\n")
            (root / "npc/pre-re/classic.txt").write_text("town mapflag pvp off\ntown mapflag gvg // classic\n")
            self.assertEqual(import_manifest(root), [{"map": "town", "flag": "pvp"},
                {"map": "town", "flag": "pvp", "enabled": False}, {"map": "town", "flag": "gvg"}])

    def test_renewal_source_and_unsupported_flags_fail_before_creating_an_output(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / "npc/pre-re").mkdir(parents=True)
            (root / "npc/scripts_mapflags.conf").write_text("npc: npc/re/ignored.txt\n")
            (root / "npc/pre-re/scripts_mapflags.conf").write_text("")
            with self.assertRaisesRegex(ValueError, "Invalid pre-renewal"):
                import_manifest(root)
            (root / "npc/scripts_mapflags.conf").write_text("npc: npc/shared.txt\n")
            (root / "npc/shared.txt").write_text("town mapflag unsupported_rule\n")
            with self.assertRaisesRegex(ValueError, "shared.txt:1: Unsupported"):
                import_manifest(root)
            self.assertFalse((root / "map_flags.json").exists())


if __name__ == "__main__":
    unittest.main()
