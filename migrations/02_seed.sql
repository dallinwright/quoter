-- Switch to the new database
USE quote_app;
GO

-- Only the author with the context can access the data, no on else can so a standard query returns nothing
INSERT INTO quote (author, quote) VALUES
    (N'Oscar Wilde', N'“Be yourself; everyone else is already taken.” ― Oscar Wilde'),
    (N'Marilyn Monroe', N'“I''m selfish, impatient and a little insecure. I make mistakes, I am out of control and at times hard to handle. But if you can''t handle me at my worst, then you sure as hell don''t deserve me at my best.” ― Marilyn Monroe'),
    (N'Frank Zappa', N'“So many books, so little time.” ― Frank Zappa'),
    (N'Albert Einstein', N'“Two things are infinite: the universe and human stupidity; and I''m not sure about the universe.” ― Albert Einstein'),
    (N'Marcus Tullius Cicero', N'“A room without books is like a body without a soul.” ― Marcus Tullius Cicero'),
    (N'Bernard M. Baruch', N'“Be who you are and say what you feel, because those who mind don''t matter, and those who matter don''t mind.” ― Bernard M. Baruch'),
    (N'William W. Purkey', N'“You''ve gotta dance like there''s nobody watching, Love like you''ll never be hurt, Sing like there''s nobody listening, And live like it''s heaven on earth.” ― William W. Purkey'),
    (N'Dr. Seuss', N'“You know you''re in love when you can''t fall asleep because reality is finally better than your dreams.” ― Dr. Seuss'),
    (N'Mae West', N'“You only live once, but if you do it right, once is enough.” ― Mae West'),
    (N'Mahatma Gandhi', N'“Be the change that you wish to see in the world.” ― Mahatma Gandhi'),
    (N'Robert Frost', N'“In three words I can sum up everything I''ve learned about life: it goes on.” ― Robert Frost'),
    (N'J.K. Rowling', N'“If you want to know what a man''s like, take a good look at how he treats his inferiors, not his equals.” ― J.K. Rowling, Harry Potter and the Goblet of Fire'),
    (N'Albert Camus', N'"In the depth of winter, I finally learned that within me there lay an invincible summer." ― Albert Camus'),
    (N'Albert Camus', N'"The only way to deal with an unfree world is to become so absolutely free that your very existence is an act of rebellion." - Albert Camus'),
    (N'Albert Camus', N'“Don’t walk in front of me… I may not follow. Don’t walk behind me… I may not lead. Walk beside me… just be my friend.” ― Albert Camus');

-- For example, view only Oscar Wilde's rows or for Albert Camus
EXEC sp_set_session_context @key = N'user_name', @value = N'Albert Camus';
GO
SELECT * FROM quote;

-- Get random quote
SELECT TOP 1 id, quote, author FROM dbo.quote WHERE author = 'Albert Camus' ORDER BY NEWID();
