class SyncBalanceDescriptor {
  private static native String balance();

  static {
    System.loadLibrary("sync_the_balance_of_a_descriptor");
  }

  public static void main(String[] args) {
    String output = SyncBalanceDescriptor.balance();
    System.out.println(output);
  }
}
