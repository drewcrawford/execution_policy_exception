# execution_policy_exception
Tired of getting 'App is damaged and can’t be opened. You should move it to the Trash' when building an app for local development?

Xcode requests a 'system policy exception' for its applications by using the `builtin-RegisterExecutionPolicyException` build step.
However, as you can tell by the name, there is no normal Unix command to do this, the logic is 'builtin' to Xcode and its build
system and cannot be run separately.

This is a standalone Unix utility that does the same thing.  Like Xcode, it requires the 'Developer Tools' security permission.

```bash
execution_policy_exception /Path/To/My.app
```

