# PathExtractTest

Extracts some information from a directory based on its structure.

The basic gist is:

- There is a magic directory, called ".run".
- Things go as "something/.run" or "something/.run/special/path".
- If we send "something/.run", we need to get a pair with path "something" and
  the name "default";
- If we send "something/.run/special/path", we need to get a pair with path
  "something and the name "path.
