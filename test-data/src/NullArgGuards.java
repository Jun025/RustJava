class NullArgGuards {
    public static void main(String[] args) throws Exception {
        // Each case passes null where the runtime previously dereferenced it without a guard,
        // which aborted the host process instead of throwing. Expect NullPointerException.

        System.out.println("arraycopy src null:");
        try {
            int[] dst = new int[1];
            System.arraycopy(null, 0, dst, 0, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("arraycopy dest null:");
        try {
            int[] srcArr = new int[1];
            System.arraycopy(srcArr, 0, null, 0, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("String(byte[]):");
        try {
            String s = new String((byte[]) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("String(char[]):");
        try {
            String s = new String((char[]) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("String(byte[],int,int):");
        try {
            String s = new String((byte[]) null, 0, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("String(byte[],String):");
        try {
            String s = new String((byte[]) null, "UTF-8");
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("String(byte[],int,int,String):");
        try {
            String s = new String((byte[]) null, 0, 0, "UTF-8");
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("String(String):");
        try {
            String s = new String((String) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("String(StringBuffer):");
        try {
            String s = new String((StringBuffer) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }
    }
}
