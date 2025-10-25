-- Create the application database
CREATE DATABASE quote_app;
GO

-- Switch to the new database
USE quote_app;
GO

-- Create the table in quote_app
CREATE TABLE dbo.quote
(
    id UNIQUEIDENTIFIER NOT NULL PRIMARY KEY DEFAULT NEWID(),
    author NVARCHAR(MAX) NOT NULL,
    quote NVARCHAR(MAX) NOT NULL,
    created_at DATETIME2 NOT NULL DEFAULT SYSUTCDATETIME()
);
GO

-- Create a login if it doesn't exist
CREATE LOGIN quote_user WITH PASSWORD = 'UserStrong!Passw0rd';
GO

-- Create the user inside quote_app database and grant minimal privileges
CREATE USER quote_user FOR LOGIN quote_user;
ALTER ROLE db_datareader ADD MEMBER quote_user;
ALTER ROLE db_datawriter ADD MEMBER quote_user;
GO

-- Create RLS schema
CREATE SCHEMA rls;
GO

-- Create RLS function
CREATE FUNCTION rls.fn_author_security(@author NVARCHAR(MAX))
    RETURNS TABLE
        WITH SCHEMABINDING
        AS
        RETURN SELECT 1 AS fn_result
               WHERE @author = CAST(SESSION_CONTEXT(N'user_name') AS NVARCHAR(MAX));
GO

-- Create the security policy
CREATE SECURITY POLICY rls.quote_policy
    ADD FILTER PREDICATE rls.fn_author_security(author) ON dbo.quote
    WITH (STATE = ON);
GO
