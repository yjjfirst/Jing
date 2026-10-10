-- This file should undo anything in `up.sql`
ALTER TABLE recordings RENAME TO sound_files;
ALTER TABLE announcements RENAME COLUMN recording_id TO sound_file_id;