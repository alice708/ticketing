CREATE TABLE events (
  id uuid PRIMARY KEY,
  name text NOT NULL,
  starts_at timestamptz NOT NULL
);

CREATE TABLE seats (
  id uuid PRIMARY KEY,
  event_id uuid NOT NULL REFERENCES events(id),
  label text NOT NULL,
  price_pence int NOT NULL,
  held_by uuid,
  held_until timestamptz,
  UNIQUE (event_id, label)
);

CREATE TABLE bookings (
  id uuid PRIMARY KEY,
  seat_id uuid NOT NULL UNIQUE REFERENCES seats(id),
  user_id uuid NOT NULL,
  idempotency_key text NOT NULL UNIQUE,
  created_at timestamptz NOT NULL DEFAULT now()
);