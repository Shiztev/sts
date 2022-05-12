package com.ssftp;
import java.util.Scanner;

import com.jcraft.jsch.ChannelExec;
import com.jcraft.jsch.JSch;
import com.jcraft.jsch.JSchException;
import com.jcraft.jsch.Session;


/**
 * A SSFTP client session.
 * 
 * @author Stevie Alvarez
 */
public class SSFTP {

  private Session session = null;
  private ChannelExec channel = null;
  private Scanner scanner = new Scanner(System.in);

  /**
   * Initialize an SSFTP session over the default SSH port. This is probably what you want to do.
   * 
   * @param username name of the account to access on the SSH server
   * @param host the hostname of the SSH server
   */
  public SSFTP(String username, String host) {
    this(username, host, 22);
  }
  
  /**
   * Initialize an SSFTP session over a unique port.
   * 
   * @param username name of the account to access on the SSH server
   * @param host the hostname of the SSH server
   * @param port the port to communicate over
   */
  public SSFTP(String username, String host, int port) {



    try {
      // create session
      this.session = new JSch().getSession(username, host, port);

      // get password
      System.out.print("password: ");
      session.setPassword(scanner.nextLine().strip());
      System.out.println();

      // configure and connect session
      session.setConfig("StrictHostKeyChecking", "no");
      session.connect();

      // create channel
      this.channel = (ChannelExec)session.openChannel("exec");

      // set up cleanup
      Runtime.getRuntime().addShutdownHook(new Thread(() -> {
        channel.disconnect();
        session.disconnect();
        System.out.println("connection terminated");
      }));

    } catch (JSchException e) {
      System.err.println(e.getLocalizedMessage());
      e.printStackTrace();
    }
  }


  /**
   * Process user input and run respective commands.
   */
  public void run() {

  }
}
