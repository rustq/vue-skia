export class SelfIncreaseCount {

    private static _count = 0;

    static get count(): number {
        return SelfIncreaseCount._count++;
    }

    static get root(): number {
        return 0;
    }
}