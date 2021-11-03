#[duplicate::duplicate(refs(T); [& T]; [T])]
fn from(x: refs(Bits<1, false>)) -> bool {
	x.value == 1
}