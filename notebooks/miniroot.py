from party.miniroot import *
from party.histogram import *


def main():
    """An example usage of `party.miniroot`
    """
    h = Hist1D(40, 0.0, 200.0)
    h.set_error_type(BinErrorType.Normal)

    r = Miniroot(r"miniroot/resources/example.root")
    array = r.get(r"Zjets/MET", DataType.Float) / 1e3

    h.fill_array(array)

    plt.figure()
    h.plot(label="Z+jets", color="black", density=True)
    plt.ylim([0, 0.1])
    plt.xlabel("MET [GeV]")
    plt.ylabel("Arbitrary Unit")
    plt.legend()
    plt.show()


if __name__ == "__main__":
    main()
