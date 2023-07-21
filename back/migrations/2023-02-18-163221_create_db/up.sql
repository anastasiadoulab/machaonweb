-- Your SQL goes here

CREATE TABLE requests
( id BIGINT NOT NULL AUTO_INCREMENT,
  reference CHAR(255) NOT NULL,
  candidates_list_id INT NOT NULL DEFAULT -1,
  custom_list TEXT NOT NULL DEFAULT "" COMMENT "Sorted comma-separated string",
  uncached TEXT NOT NULL DEFAULT "" COMMENT "Sorted comma-separated string",
  hash_value CHAR(255) NOT NULL,
  creation_date DATETIME NOT NULL DEFAULT NOW(),
  meta BOOLEAN DEFAULT 0 NOT NULL,
  go_term CHAR(255) NOT NULL DEFAULT "",
  comparison_mode TINYINT NOT NULL DEFAULT 0 COMMENT "Whole structure (0), Domain (1), Segment (2)",
  segment_start INT NOT NULL DEFAULT -1,
  segment_end INT NOT NULL DEFAULT -1,
  alignment_level TINYINT NOT NULL DEFAULT 0 COMMENT "1D (0), 2D (1), hydrophobicity (2), mixed (3)",
  views BIGINT NOT NULL DEFAULT 0,
  CONSTRAINT requests_pk PRIMARY KEY (id)
);

CREATE TABLE jobs
( id BIGINT NOT NULL AUTO_INCREMENT,
  request_id BIGINT NOT NULL,
  node_id TINYINT NOT NULL,
  assignment_date DATETIME NOT NULL DEFAULT NOW(),
  completion_date DATETIME,
  last_checked DATETIME,
  status_code TINYINT NOT NULL DEFAULT -1,
  secure_hash CHAR(255) NOT NULL DEFAULT '', 
  CONSTRAINT jobs_pk PRIMARY KEY (id)
);

CREATE TABLE nodes
( id TINYINT NOT NULL AUTO_INCREMENT,
  ip CHAR(255) NOT NULL,
  domain CHAR(255) NOT NULL DEFAULT '', 
  active BOOLEAN DEFAULT 0 NOT NULL,
  working BOOLEAN DEFAULT 0 NOT NULL,
  cores TINYINT NOT NULL,
  sync_date DATETIME NOT NULL DEFAULT NOW(),
  CONSTRAINT nodes_pk PRIMARY KEY (id)
);

CREATE TABLE cached_features
( id BIGINT NOT NULL AUTO_INCREMENT,
  structure_id CHAR(255) NOT NULL,
  CONSTRAINT cached_features_pk PRIMARY KEY (id)
);

CREATE TABLE candidate_lists
( id INT NOT NULL AUTO_INCREMENT,
  title CHAR(255) NOT NULL,
  CONSTRAINT candidate_lists_pk PRIMARY KEY (id)
);

CREATE INDEX structure_id_index ON cached_features(structure_id);
CREATE INDEX hash_value_index ON requests(hash_value);
CREATE INDEX completion_date_index ON jobs(completion_date);
CREATE INDEX last_checked_index ON jobs(last_checked);
CREATE INDEX request_id_index ON jobs(request_id);
CREATE INDEX assignment_date_index ON jobs(assignment_date);
CREATE INDEX sync_date_index ON nodes(sync_date);

INSERT INTO candidate_lists (title)
VALUES ("Pub_Viral_PDB_Candidate_Set (41674 structures)"),  
("Human_PDB_Candidate_Set_v1 (161629 structures)"),
("Human_AF4_Candidate_Set");

INSERT INTO nodes (ip, active, cores)
VALUES ("localhost:55555", 1, 1);

