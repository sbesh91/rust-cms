SELECT 'CREATE DATABASE cms_db'
WHERE NOT EXISTS 
(SELECT FROM pg_database WHERE datname = 'cms_db')\gexec