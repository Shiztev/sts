package com.ssftp;

/**
 * Combine SSH and SFTP utility to easily navigate a system and manage files.
 * 
 * @author Stevie Alvarez
 */
public class App {
  public static void main( String[] args ) {
    // TODO: parse args <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
    // if port provided, call correct SSFTP constructor
    
    SSFTP ssftp = new SSFTP(username, host);
    ssftp.run();
  }
}
