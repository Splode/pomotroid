#!/usr/bin/env python3
"""Seed the Pomotroid database with realistic historical session data.

Usage:
    python3 scripts/seed-db.py [options]

Options:
    --days N      Days of history to generate (default: 730 ≈ 2 years)
    --db PATH     Override the database path
    --clear       Delete all existing sessions before inserting
    --dry-run     Print a summary without writing to the database
    --seed N      Random seed for reproducible output (default: 42)

The script generates realistic Pomodoro session sequences — work rounds,
short breaks, and long breaks — with natural weekday/weekend variation,
streaks, and quiet periods. Sessions are in proper 4-work-per-cycle order
matching Pomotroid's default sequence.

The database must already exist (launch Pomotroid at least once first).
"""

import argparse
import os
import platform
import random
import sqlite3
import sys
from datetime import date, datetime, timedelta


# ── Platform database path ────────────────────────────────────────────────────

APP_ID = "com.splode.pomotroid"
DB_NAME = "pomotroid.db"


def default_db_path() -> str:
    system = platform.system()
    if system == "Linux":
        base = os.environ.get("XDG_DATA_HOME", os.path.expanduser("~/.local/share"))
        return os.path.join(base, APP_ID, DB_NAME)
    if system == "Darwin":
        return os.path.expanduser(
            f"~/Library/Application Support/{APP_ID}/{DB_NAME}"
        )
    if system == "Windows":
        appdata = os.environ.get("APPDATA", "")
        return os.path.join(appdata, APP_ID, DB_NAME)
    print(f"Unsupported platform: {system}", file=sys.stderr)
    sys.exit(1)


# ── Session generation ────────────────────────────────────────────────────────

WORK_SECS  = 1500   # 25 minutes
SHORT_SECS = 300    #  5 minutes
LONG_SECS  = 900    # 15 minutes

ROUNDS_PER_CYCLE = 4  # Work rounds before a long break


def make_session(ts: int, duration: int, round_type: str, completed: int = 1) -> dict:
    return {
        "started_at":    ts,
        "ended_at":      ts + duration,
        "round_type":    round_type,
        "duration_secs": duration,
        "completed":     completed,
    }


def generate_sessions(days: int, rng: random.Random) -> list[dict]:
    """Generate realistic session history for `days` days ending today."""
    sessions = []
    today     = date.today()
    start_day = today - timedelta(days=days - 1)

    for offset in range(days):
        day     = start_day + timedelta(days=offset)
        weekday = day.weekday()   # 0 = Monday … 6 = Sunday

        # Weekday/weekend activity profiles
        if weekday >= 5:   # weekend
            if rng.random() > 0.28:
                continue
            num_cycles = rng.randint(1, 2)
        else:              # weekday
            if rng.random() > 0.78:
                continue
            # Weight toward 2–3 cycles; occasional marathon day
            num_cycles = rng.choices([1, 2, 3, 4], weights=[10, 35, 40, 15])[0]

        # Session start time: between 8 am and 1 pm local time
        hour   = rng.randint(8, 13)
        minute = rng.choice([0, 15, 30, 45])
        ts     = int(datetime(day.year, day.month, day.day, hour, minute).timestamp())

        for cycle in range(num_cycles):
            for work_idx in range(ROUNDS_PER_CYCLE):
                # Work round
                sessions.append(make_session(ts, WORK_SECS, "work"))
                ts += WORK_SECS

                is_last_work  = work_idx == ROUNDS_PER_CYCLE - 1
                is_last_cycle = cycle == num_cycles - 1

                if is_last_work:
                    # Long break ends the cycle — omit after the very last work round
                    if not is_last_cycle:
                        sessions.append(make_session(ts, LONG_SECS, "long-break"))
                        ts += LONG_SECS
                else:
                    # Short break between work rounds (occasionally skipped)
                    if rng.random() > 0.08:
                        sessions.append(make_session(ts, SHORT_SECS, "short-break"))
                    ts += SHORT_SECS

            # Brief gap between cycles (5–20 min of non-Pomotroid time)
            if not is_last_cycle:
                ts += rng.randint(5, 20) * 60

    return sessions


# ── Database I/O ──────────────────────────────────────────────────────────────

INSERT_SQL = """
    INSERT INTO sessions (started_at, ended_at, round_type, duration_secs, completed)
    VALUES (:started_at, :ended_at, :round_type, :duration_secs, :completed)
"""


def summarise(sessions: list[dict]) -> None:
    work_sessions  = [s for s in sessions if s["round_type"] == "work"]
    short_sessions = [s for s in sessions if s["round_type"] == "short-break"]
    long_sessions  = [s for s in sessions if s["round_type"] == "long-break"]
    focus_hours    = sum(s["duration_secs"] for s in work_sessions) / 3600

    active_days = len({
        date.fromtimestamp(s["started_at"])
        for s in work_sessions
    })

    print(f"  Total sessions : {len(sessions)}")
    print(f"    Work         : {len(work_sessions)}")
    print(f"    Short breaks : {len(short_sessions)}")
    print(f"    Long breaks  : {len(long_sessions)}")
    print(f"  Active days    : {active_days}")
    print(f"  Focus time     : {focus_hours:.1f} hours")


def seed(db_path: str, sessions: list[dict], *, clear: bool, dry_run: bool) -> None:
    print(f"Database : {db_path}")
    summarise(sessions)

    if dry_run:
        print("\nDry run — no changes written.")
        return

    if not os.path.exists(db_path):
        print(
            f"\nError: database not found at:\n  {db_path}\n"
            "Launch Pomotroid at least once to create the database, then re-run.",
            file=sys.stderr,
        )
        sys.exit(1)

    with sqlite3.connect(db_path) as conn:
        if clear:
            deleted = conn.execute("DELETE FROM sessions").rowcount
            print(f"\nCleared  : {deleted} existing session(s) deleted")

        conn.executemany(INSERT_SQL, sessions)
        print(f"\nInserted : {len(sessions)} sessions")


# ── Entry point ───────────────────────────────────────────────────────────────

def main() -> None:
    parser = argparse.ArgumentParser(
        description="Seed the Pomotroid database with test session history.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__.strip(),
    )
    parser.add_argument(
        "--days", type=int, default=730, metavar="N",
        help="Days of history to generate (default: 730 ≈ 2 years)",
    )
    parser.add_argument(
        "--db", metavar="PATH",
        help="Override database path (default: platform app-data location)",
    )
    parser.add_argument(
        "--clear", action="store_true",
        help="Delete all existing sessions before inserting",
    )
    parser.add_argument(
        "--dry-run", action="store_true",
        help="Print a summary without writing to the database",
    )
    parser.add_argument(
        "--seed", type=int, default=42, metavar="N",
        help="Random seed for reproducible output (default: 42)",
    )

    args = parser.parse_args()

    db_path  = args.db or default_db_path()
    rng      = random.Random(args.seed)
    sessions = generate_sessions(args.days, rng)

    seed(db_path, sessions, clear=args.clear, dry_run=args.dry_run)


if __name__ == "__main__":
    main()
