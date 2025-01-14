sudo bpftrace -e '
kprobe:input_event {
    // Capture the event information
    printf("Key event detected: type=%d, code=%d, value=%d\n",
           arg2, arg3, arg4);

    // Capture the current PID
    printf("  PID: %d\n", pid);
    printf("  UID: %d\n", uid);

    // Capture the TTY associated with the process
    printf("  TTY: %s\n", curtask->signal->tty->name);
}'
