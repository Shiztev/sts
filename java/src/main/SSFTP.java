import com.jcraft.jsch.ChannelExec;
import com.jcraft.jsch.JSch;
import com.jcraft.jsch.Session;


/**
 * A SSFTP client session.
 * 
 * @author Stevie Alvarez
 */
public class SSFTP {

  private Session session = null;
  private ChannelExec channel = null;

  /**
   * Initialize an SSFTP session over the default SSH port. This is probably what you want to do.
   * 
   * @param username name of the account to access on the SSH server
   * @param host the hostname of the SSH server
   */
  public SSFTP(String username, String host) {
    super(username, host, 22);
  }
  
  /**
   * Initialize an SSFTP session over a unique port.
   * 
   * @param username name of the account to access on the SSH server
   * @param host the hostname of the SSH server
   * @param port the port to communicate over
   */
  public SSFTP(String username, String host, int port) {

    
  }
}
