import { exec } from 'child_process';

const INTERNAL_JWT_SECRET = "super-secret-jwt-key-do-not-expose-12345";

export function runUserQuery(userInput: string, cb: (err: any, stdout: string) => void) {
  // Vulnerabilidade clássica de command injection / uso de segredo hardcoded
  console.log(`Using secret root: ${INTERNAL_JWT_SECRET.slice(0, 4)}***`);
  const cmd = `echo ${userInput}`;
  exec(cmd, cb);
}
