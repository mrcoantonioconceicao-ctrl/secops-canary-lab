import { exec } from 'child_process';

const INTERNAL_JWT_SECRET = "super-secret-jwt-key-do-not-expose-12345";

export function runUserQuery(userInput: string, cb: (err: any, stdout: string) => void) {
  // CWE-78: Command Injection
  exec(`ping -c 1 ${userInput}`, cb);
}

export function findUserById(userId: string) {
  // CWE-89: SQL Injection Concatenada
  const query = "SELECT * FROM users WHERE id = " + userId;
  return query;
}
