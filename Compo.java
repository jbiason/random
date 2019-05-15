/*
 * This is me testing function composition in Java.
 *
 */
import java.util.List;
import java.util.stream.Stream;
import java.util.stream.Collectors;
import java.util.function.Function;
import java.util.function.Consumer;

public class Compo {
	public static void main(String[] argv) {
		System.out.println("Starting up...");

		Doubler doubler = new Doubler();
		PlusTwo plusTwo = new PlusTwo();
		Printer printer = new Printer();

		List<Long> result = Stream.of(2L, 4L, 5L, 7L)
			.peek(doubler.andThen(plusTwo).andThen(printer))
			.collect(Collectors.toList());
		System.out.println("Result: " + result);
	}

	private static class Doubler implements Function<Long, Long> {
		@Override
		public Long apply(Long input) {
			return input * 2;
		}
	}

	private static class PlusTwo implements Function<Long, Long> {
		@Override
		public Long apply(Long input) {
			return input + 2L;
		}
	}

	private static class Printer implements Consumer<Long> {
		@Override
		public void accept(Long input) {
			System.out.println("Consumer: " + input);
		}
	}
}
