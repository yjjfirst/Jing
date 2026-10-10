-- Your SQL goes here
ALTER TABLE sound_files RENAME TO recordings;
ALTER TABLE announcements RENAME COLUMN sound_file_id TO recording_id;