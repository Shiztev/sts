package com.ssftp;
import java.io.Console;
import java.io.IOException;
import java.lang.reflect.Field;
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

  /**
   * The domain/ip of the host server.
   */
  private String host = null;

  /**
   * The username of the account on the host server.
   */
  private String user = null;

  /**
   * Network {@link Session} with the host on some arbitrary port.
   */
  private Session session = null;

  /**
   * The {@link Channel} that can be used to communicate with the server.
   */
  private ChannelExec channel = null;

  /**
   * {@link Scanner} for user input from standard input.
   */
  private static Scanner scanner = new Scanner(System.in);

  /**
   * THe {@link Console} IO of the system.
   */
  private static Console console;

  /**
   * Initialize an SSFTP session over the default SSH port. This is probably what you want to do.
   * 
   * @param username name of the account to access on the SSH server
   * @param host the hostname of the SSH server
   * @throws IOException
   */
  public SSFTP(String username, String host) throws IOException {
    this(username, host, 22);
  }
  
  /**
   * Initialize an SSFTP session over a unique port.
   * 
   * @param username name of the account to access on the SSH server
   * @param host the hostname of the SSH server
   * @param port the port to communicate over
   * @throws IOException
   */
  public SSFTP(String username, String host, int port) throws IOException {

    this.user = username;
    this.host = host;

    if (((console = System.console())) == null) {
      System.err.println("error: cannot access system console");
      throw new IOException();
    }

    try {
      // create session
      this.session = new JSch().getSession(username, host, port);

      // get password
      System.out.print("password: ");
      char[] pw = console.readPassword();
      String p = new String(pw);
      session.setPassword(p);

      // attempt to wipe password from memory (the jvm's making me rip my hair out... you can't actually remove memory ;-;)
      new Thread(() -> {
        // zero out char array
        for (int i = 0; i < pw.length; ++i) {
          pw[i] = 0;
        }

        Field field;

        try {
          field = String.class.getDeclaredField("value");
          field.setAccessible(true);

          String value = new String();
          for (int i = 0; i < p.length(); ++i) {
            value += "0";
          }

          field.set(p, value);

        } catch (NoSuchFieldException e) {
          e.printStackTrace();
        } catch (SecurityException e) {
          e.printStackTrace();
        } catch (IllegalArgumentException e) {
          e.printStackTrace();
        } catch (IllegalAccessException e) {
          e.printStackTrace();
        }
        
      }).run();

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
