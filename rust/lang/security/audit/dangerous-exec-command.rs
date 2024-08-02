fn dangerous_exec_command_sh() {
    use std::process::Command;
    let shell = "sh";
    let cmd = "echo hello";
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell);
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
}

fn dangerous_exec_command_bash() {
    use std::process::Command;
    let shell = "bash";
    let cmd = "echo hello";
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell);
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
 
}

fn dangerous_exec_command_csh() {
    use std::process::Command;
    let shell = "csh";
    let cmd = "echo hello";
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell);
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

}

fn dangerous_exec_command_ksh() {
    use std::process::Command;
    let shell = "ksh";
    let cmd = "echo hello";
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell);
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

}

fn dangerous_exec_command_tcsh() {
    use std::process::Command;
    let shell = "tcsh";
    let cmd = "echo hello";
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell);
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
}

fn dangerous_exec_command_zsh() {
    use std::process::Command;
    let shell = "zsh";
    let cmd = "echo hello";
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell);
    //ruleid: dangerous-exec-command
    let _ = Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
    
    let input = "ls"
    //ruleid: dangerous-exec-command
    let mut ls = Command::new(input);
}
