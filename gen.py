import itertools

def flatten(xss):
    return [x for xs in xss for x in xs]

ll = []

for i in range(2, 5):
	l = list(map(lambda x: ''.join(x), itertools.combinations_with_replacement('xyzw', i)))
	l = set(flatten(
		list(map(
			lambda x: list(
				map(
					lambda y: ''.join(y),
					itertools.permutations(x)
				)
			), l
		)
	)))

	ll.append(list(l))

	print(len(l))

ll = flatten(ll)
print(ll, len(ll))
