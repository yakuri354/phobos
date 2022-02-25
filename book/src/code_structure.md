# Code structure

The kernel has a strict structure to make navigation easy. It obeys the following rules:
- All architecture-specific code goes to `arch`
- All device-specific code, including drivers goes to `device`
- All other code gets divided into arch-independent modules based on its purpose

Every module has module-level documentation explaining its purpose.