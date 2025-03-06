// @generated file from wasmbuild -- do not edit
// deno-lint-ignore-file
// deno-fmt-ignore-file

export function hash_password(
  password: string,
  memory_cost?: number | null,
  time_cost?: number | null,
  output_len?: number | null,
  parallelism?: number | null,
): string;
export function verify_password(
  password: string,
  hash: string,
  memory_cost?: number | null,
  time_cost?: number | null,
  output_len?: number | null,
  parallelism?: number | null,
): boolean;
