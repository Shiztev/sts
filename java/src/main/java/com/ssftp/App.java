package com.ssftp;

import java.io.IOException;

/**
 * Combine SSH and SFTP utility to easily navigate a system and manage files.
 * 
 * @author Stevie Alvarez
 */
public class App {
  public static void main( String[] args ) throws IOException {
    String username = "test";
    String host = "this.place.dne.com";
    // TODO: parse args <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
    // if port provided, call correct SSFTP constructor
    
    SSFTP ssftp = new SSFTP(username, host);
    ssftp.run();
  }
}
