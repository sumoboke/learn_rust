CREATE TABLE trucks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  time_submit BIGINT NOT NULL DEFAULT (strftime('%s', 'now')),
  vehicle_num TEXT NOT NULL,
  driver_name TEXT NOT NULL, 
  from_location TEXT NOT NULL,
  to_location TEXT NOT NULL,
  time_drive BIGINT NOT NULL,
  is_done BOOLEAN NOT NULL DEFAULT FALSE
);
